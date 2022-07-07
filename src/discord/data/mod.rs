use serenity::model::prelude::GuildId;
use std::collections::HashMap;
use crate::database::DoloredDatabase;
use crate::discord::data::config::ConfigurationEntry;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

mod config;
pub use config::load_config;
use reqwest::Client;
use skynet_api::apis::configuration::Configuration;

pub type BotConfig = HashMap<GuildId, ConfigurationEntry>;

pub struct BotData {
    pub db: DoloredDatabase,
    pub config: BotConfig,
    pub client: Client,
    pub skynet: Configuration,
}


pub struct DataKey;

impl TypeMapKey for DataKey {
    type Value = Arc<BotData>;
}


