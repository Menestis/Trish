use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Pool, MySql};
use sqlx::migrate::Migrator;
use std::path::Path;
use log::*;

pub mod queries;

pub async fn init(uri: &str) -> Result<DoloredDatabase, sqlx::Error>{

    let pool = MySqlPoolOptions::new().max_connections(4).connect(uri).await?;
    debug!("Running migrations");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)

}

pub type DoloredDatabase = Pool<MySql>;