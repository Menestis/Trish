use std::collections::HashMap;
use crate::discord::data::BotConfig;
use std::{fs, io};
use serenity::model::prelude::{ChannelId, MessageId, RoleId};
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct ConfigurationEntry {
    pub name: String,
    pub captcha: Option<Captcha>,
    pub logs: Option<Logs>,
    #[serde(default)]
    pub publish: Vec<ChannelId>,
    pub allowed_channels: Vec<ChannelId>,
    #[serde(default)]
    pub roles_menus: Vec<RoleMenu>,
    pub games: Option<Games>
}


#[derive(Serialize, Deserialize)]
pub struct RoleMenu {
    pub channel: ChannelId,
    pub message: MessageId,
    #[serde(default)]
    pub inverted: bool,
    pub reactions: HashMap<String, RoleId>,
}

#[derive(Serialize, Deserialize)]
pub struct Captcha {
    pub category: ChannelId,
    pub message: String,
    pub channel_name: String,
    pub rank: RoleId,
    pub fail_message: String,
    pub welcome_message: String,
    pub after_rank: RoleId,
    pub log_channel: Option<ChannelId>,
}

#[derive(Serialize, Deserialize)]
pub struct Logs {
    pub channel: ChannelId,
}

#[derive(Serialize, Deserialize)]
pub struct Games {
    pub publish_channel: ChannelId,
    pub publish_mention: RoleId,
    pub confirmation_channel: ChannelId,
    pub confirmation_roles: Vec<RoleId>,
    pub game_category: ChannelId
}

pub fn load_config(path: &str) -> Result<BotConfig, ConfigurationError> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Could not read config: {0}")]
    IO(#[from] io::Error),
    #[error("Could not deserialize config: {0}")]
    SERDE(#[from] serde_json::Error),
}