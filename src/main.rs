use axum::extract::State;
use axum::response::IntoResponse;
use axum::{debug_handler, routing, Json, Router};
use rust_axum::{ database, logger};
use rust_axum::entity::{prelude::*, sys_user};
use sea_orm::{Condition, DatabaseConnection, EntityTrait, QueryFilter};
use tokio::{net::TcpListener};
use rust_axum::config;
use sea_orm::ColumnTrait;

// Axum web 开发三部曲 
//
// 1. 路由
// 2. 监听
// 3. 服务

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    // 初始化日志
    logger::init();

    let db = database::init().await?;

    // 构建路由
    let routes = Router::new()
        .route("/", routing::get(index))
        .route("/user", routing::get(query_users))
        .with_state(db);

    let port = config::get().server().port();
    // 监听端口
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    tracing::info!("Listening on http://127.0.0.1:{port}");
    // 3. 开启服务
    axum::serve(listener, routes).await?;
    Ok(())
}


// 原始的错误信息并不明确，需要结束这个宏来debug 
// 帮助打印发生异常时候的错误信息，方便分析问题
// 这个在打发行包的时候不会编译，不会带来生产环境开销
#[debug_handler]
async fn index() -> &'static str {
    "Hello world"
}

#[debug_handler]
async fn query_users(State(db): State<DatabaseConnection>) -> impl IntoResponse {
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
    Json(users)
}
