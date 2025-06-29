use anyhow::Context;
use crate::entity::sys_user::ActiveModel;
use crate::enums::Gender;
use crate::param_valid::Path;
use axum::{Router, debug_handler, routing};
use axum::extract::State;
use sea_orm::{prelude::*, ActiveValue, IntoActiveModel};
use sea_orm::{
    ColumnTrait, Condition, DeriveIntoActiveModel, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QueryTrait
};
use serde::Deserialize;
use validator::Validate;

use crate::app::AppState;
use crate::common::{Page, PaginationParams};
use crate::entity::{prelude::SysUser, sys_user};
use crate::error::{ApiError, ApiResult};
use crate::response::ApiResponse;
use crate::valid::{ValidJson, ValidQuery};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(query_users))
        .route("/page", routing::get(page_user))
        .route("/create", routing::post(create_user))
        .route("/update/{id}", routing::put(update_user))
        .route("/delete/{id}", routing::delete(delete_user))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    keyword: Option<String>,

    // 嵌套校验内层结构
    #[validate(nested)]
    #[serde(flatten)]
    pagination: PaginationParams,
}

// 原始的错误信息并不明确，需要结束这个宏来debug
// 帮助打印发生异常时候的错误信息，方便分析问题
// 这个在打发行包的时候不会编译，不会带来生产环境开销
#[debug_handler]
async fn query_users(
    State(AppState { db }): State<AppState>,
) -> ApiResult<ApiResponse<Vec<sys_user::Model>>> {
    let users = SysUser::find()
        // filter only one condition
        // sea_orm::condition compose multiple conditions
        .filter(
            Condition::all()
                .add(sys_user::Column::Gender.eq("female"))
                .add(sys_user::Column::Name.starts_with("A"))
                .add(Condition::any().add(sys_user::Column::Enabled.eq(false))),
        )
        .all(&db)
        .await
        .context("query users error")?;

    Ok(ApiResponse::ok("ok", Some(users)))
}

#[debug_handler]
async fn page_user(
    State(AppState { db }): State<AppState>,
    // Query 抽取器取出参数
    // Valid 将抽取出来的结果进行校验
    ValidQuery(UserQueryParams {
        keyword,
        pagination,
    }): ValidQuery<UserQueryParams>
) -> ApiResult<ApiResponse<Page<sys_user::Model>>> {
    let paginator = SysUser::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .add(sys_user::Column::Name.contains(keyword))
                    .add(sys_user::Column::Account.contains(keyword)),
            )
        })
        .order_by_desc(sys_user::Column::CreatedAt)
        .paginate(&db, pagination.size);

    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page - 1).await?;
    let page = Page::from_pagination(pagination, total, items);

    Ok(ApiResponse::ok("ok", Some(page)))
}

#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
pub struct UserParams {

    #[validate(length(min = 2, max = 20, message = "姓名长度1-20"))]
    pub name: String,
    pub gender: Gender,
    #[validate(length(min = 1, max = 20, message = "账号长度1-20"))]
    pub account: String,

    // 常见的简单校验器已经内置
    #[validate(length(min = 6, max = 20, message = "密码长度6-20"))]
    pub password: String,

    // 自定义的校验器，校验手机号, 指定方法，方法中返回的错误将作为校验失败的错误
    #[validate(custom(function = "crate::utils::validation::is_mobile_phone"))]
    pub mobile_phone: String,

    pub birthday: Date,
    #[serde(default)]
    pub enabled: bool,
}

#[debug_handler]
pub async fn create_user(
    State(AppState { db }): State<AppState>,
    ValidJson(user_params): ValidJson<UserParams>
) -> ApiResult<ApiResponse<sys_user::Model>> {
    let mut user_model  = user_params.into_active_model();
    user_model.password = ActiveValue::Set(
        bcrypt::hash(
            &user_model.password.take().unwrap(),
            bcrypt::DEFAULT_COST
        )?
    );

    let result = user_model.insert(&db).await?;
    Ok(ApiResponse::ok("ok", Some(result)))
}

#[debug_handler]
pub async fn update_user(
    State(AppState { db }): State<AppState>,
    Path(id):Path<String>,
    ValidJson(user_params): ValidJson<UserParams>
) -> ApiResult<ApiResponse<sys_user::Model>> {
    let existed_user = SysUser::find_by_id(id).one(&db).await?
        .ok_or_else(|| ApiError::Biz(String::from("待修改用户不存在")))?;
    let pwd = user_params.password.clone();
    let mut active_model = user_params.into_active_model();
    if pwd.is_empty() {
        active_model.password = ActiveValue::unchanged(existed_user.password);
    }else {
        active_model.password = ActiveValue::Set(bcrypt::hash(
            &active_model.password.take().unwrap(),
            bcrypt::DEFAULT_COST
        )?);
    }
    let result =active_model.update(&db).await?;

    Ok(ApiResponse::ok("ok", Some(result)))
}   

#[debug_handler]
pub async fn delete_user(
    State(AppState { db }): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<ApiResponse<()>> { 

    let exists_user = SysUser::find_by_id(&id)
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("待删除用户不存在")))?;

    // effect rows
    let result = exists_user.delete(&db).await?;
    tracing::info!("delete user: {}, rows: {}", id, result.rows_affected);
    Ok(ApiResponse::ok("ok", None))
}

