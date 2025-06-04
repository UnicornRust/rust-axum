use sea_orm::DatabaseConnection;

use crate::{database, config, logger, server::Server};


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
    tracing::info!("Starting app server...");

    let db = database::init().await?;
    let state = AppState::new(db);
    let server = Server::new(config::get().server());

    server.start(state, router).await
}

