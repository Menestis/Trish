use crate::database::DoloredDatabase;
use std::collections::HashSet;
use serenity::http::Http;
use log::*;
use serenity::framework::StandardFramework;
use crate::discord::events::Handler;
use serenity::Client;
use crate::discord::data::{DataKey, BotData};
use std::sync::Arc;
use serenity::prelude::GatewayIntents;
use skynet_api::apis::configuration::Configuration;

mod modules;
mod events;
mod commands;
mod data;

use commands::*;

pub async fn run(db: DoloredDatabase, token: &str, config_path: &str, skynet: Configuration) -> anyhow::Result<()> {
    let config = data::load_config(config_path)?;
    let http = Http::new(token);

    let (owners, bot_id) = {
        let info = http.get_current_application_info().await?;

        let mut owners = HashSet::new();
        if let Some(team) = info.team {
            owners.insert(team.owner_user_id);
        } else {
            owners.insert(info.owner.id);
        }

        (owners, http.get_current_user().await?)
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(bot_id.id))
            .prefix("$")
            .owners(owners))
        .after(after)
        .help(&HELP)
        .group(&GENERAL_GROUP)
        .group(&ADMIN_GROUP);

    let mut client = Client::builder(token, GatewayIntents::all()).raw_event_handler(Handler)
        .framework(framework).await?;
    {
        let mut guard = client.data.write().await;
        let client = reqwest::Client::new();
        guard.insert::<DataKey>(Arc::new(BotData { db, config, client, skynet }));
    }

    info!("Running !");
    client.start().await?;
    Ok(())
}