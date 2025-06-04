use axum::{debug_handler, extract::State, routing, Router};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::app::AppState; 
use crate::entity::{prelude::SysUser, sys_user};
use crate::error::ApiResult;
use crate::response::ApiResponse;


pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(query_users))
}
// 原始的错误信息并不明确，需要结束这个宏来debug 
// 帮助打印发生异常时候的错误信息，方便分析问题
// 这个在打发行包的时候不会编译，不会带来生产环境开销

#[debug_handler]
async fn query_users(State(AppState{ db }): State<AppState>) -> ApiResult<ApiResponse<Vec<sys_user::Model>>> {
    let users = SysUser::find()
        // filter only one condition 
        // sea_orm::condition compose multiple conditions
        .filter(
            Condition::all()
            .add(sys_user::Column::Gender.eq("female"))
            .add(sys_user::Column::Name.starts_with("A"))
            .add(
                Condition::any().add(sys_user::Column::Enabled.eq(false))
            )
        )
        .all(&db)
        .await
        .unwrap();
    Ok(ApiResponse::ok("ok", Some(users)))
}
