use std::{cmp::{max, min}, time::Duration};

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use crate::config;


pub async fn init() -> anyhow::Result<DatabaseConnection> {

    let db_config = config::get().database();
    let mut options = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.username(),
        db_config.password(),
        db_config.host(),
        db_config.port(),
        db_config.database()
    ));

    let cpus = num_cpus::get() as u32;

    options.min_connections(max(cpus * 4, 10))
        .max_connections(min(cpus * 8, 20))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(60))
        .max_lifetime(Duration::from_secs(300))
        .sqlx_logging(false)
        .set_schema_search_path(db_config.schema());

    let db = Database::connect(options).await?;

    db.ping().await?;
    tracing::info!("Connected to database");

    log_database_version(&db).await?;
    Ok(db)
} 

async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()>{

    let version = db.query_one( 
        Statement::from_string( 
            DbBackend::Postgres, 
            String::from("Select version()")
        )
    ).await?
    .ok_or_else(|| anyhow::anyhow!("Failed to get database version"))?;

    tracing::info!("Database version: {}", version.try_get_by_index::<String>(0)?);
    Ok(())
}
