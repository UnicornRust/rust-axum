use rust_axum::{ framework, routes};

// Axum web 开发三部曲 
//
// 1. 路由
// 2. 监听
// 3. 服务

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    framework::run(routes::create_router()).await
}
