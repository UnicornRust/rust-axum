use anyhow::Context;
use axum::{Router, debug_handler, routing};
use axum::extract::State;
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QueryTrait,
};
use serde::Deserialize;
use validator::Validate;

use crate::app::AppState;
use crate::common::{Page, PaginationParams};
use crate::entity::{prelude::SysUser, sys_user};
use crate::error::ApiResult;
use crate::param_valid::Query;
use crate::response::ApiResponse;
use crate::valid::ValidQuery;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(query_users))
        .route("/page", routing::get(page_user))
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
