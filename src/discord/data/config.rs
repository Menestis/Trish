use std::collections::HashMap;
use crate::discord::data::BotConfig;
use std::{fs, io};
use serenity::model::prelude::{ChannelId, MessageId, RoleId, UserId};
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
    pub can_link: bool,
    #[serde(default)]
    pub roles_menus: Vec<RoleMenu>,
    #[serde(default)]
    pub games: Vec<Games>,
    #[serde(default)]
    pub message_logs: Option<DiscordLogs>
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
    pub after_rank: Option<RoleId>,
    pub log_channel: Option<ChannelId>,
}

#[derive(Serialize, Deserialize)]
pub struct Logs {
    pub channel: ChannelId,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordLogs {
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