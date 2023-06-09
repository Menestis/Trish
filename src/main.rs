use log::*;
use std::env;
use std::env::VarError;
use anyhow::Error;
use skynet_api::apis::configuration::ApiKey;

mod database;
mod discord;

#[tokio::main]
async fn main() {
    env_logger::init_from_env("LOG_LEVEL");
    info!("Starting trish v2 (Impact)");


    info!("Connecting to database !");

    let db_string = match env::var("DATABASE_URL") {
        Ok(addr) => { addr }
        Err(err) => {
            error!("DATABASE environment variable wasn't provided");
            return;
        }
    };


    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => { token }
        Err(err) => {
            error!("DISCORD_TOKEN environment variable wasn't provided");
            return;
        }
    };

    let config_path = match env::var("CONFIG_PATH") {
        Ok(path) => { path }
        Err(err) => {
            error!("CONFIG_PATH environment variable wasn't provided");
            return;
        }
    };

    let skynet_url = match env::var("SKYNET_URL") {
        Ok(skynet_url) => { skynet_url }
        Err(err) => {
            error!("SKYNET_URL environment variable wasn't provided");
            return;
        }
    };


    let skynet_key = match env::var("SKYNET_KEY") {
        Ok(skynet_key) => { skynet_key }
        Err(err) => {
            error!("SKYNET_KEY environment variable wasn't provided");
            return;
        }
    };


    let db = match database::init(&db_string).await {
        Ok(db) => { db }
        Err(err) => {
            error!("Could not connect to database: {}", err);
            return;
        }
    };


    let mut skynet = skynet_api::apis::configuration::Configuration::new();
    skynet.api_key = Some(ApiKey{ prefix: None, key: skynet_key });
    skynet.base_path = skynet_url;


    match discord::run(db, &token, &config_path, skynet).await {
        Ok(()) => info!("Bye !"),
        Err(e) => error!("Could not run trish: {}", e)
    };

}
