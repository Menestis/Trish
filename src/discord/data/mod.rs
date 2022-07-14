use serenity::model::prelude::GuildId;
use std::collections::HashMap;
use crate::database::DoloredDatabase;
use crate::discord::data::config::ConfigurationEntry;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use arraydeque::{ArrayDeque, Wrapping};

mod config;
pub use config::load_config;
use reqwest::Client;
use serenity::model::id::MessageId;
use skynet_api::apis::configuration::Configuration;
use tokio::sync::RwLock;

pub type BotConfig = HashMap<GuildId, ConfigurationEntry>;

pub struct BotData {
    pub db: DoloredDatabase,
    pub config: BotConfig,
    pub client: Client,
    pub skynet: Configuration,
    pub last_messages: RwLock<(ArrayDeque<[MessageId; 100], Wrapping>, HashMap<MessageId, String>)>
}


pub struct DataKey;

impl TypeMapKey for DataKey {
    type Value = Arc<BotData>;
}


