pub mod auth;
pub mod request;
pub mod response;
pub mod error;
pub mod common;
pub mod db;
pub mod serde;
pub mod utils;
pub mod server;
pub mod middleware;

use sea_orm::DatabaseConnection;

use crate::config;
use crate::framework::{db::database, middleware::logger, server::Server, utils::generator};


#[derive(Clone)]
pub struct AppState{
    pub db: DatabaseConnection,
}

impl AppState {

    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(router: axum::Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    generator::init()?;
    tracing::info!("Starting app server...");

    let db = database::init().await?;
    let state = AppState::new(db);
    let server = Server::new(config::get().server());

    server.start(state, router).await
}

