use crate::discord::data::BotData;
use captcha::{gen, Difficulty};
use log::*;
use reqwest::header::{AUTHORIZATION, CONTENT_LENGTH};
use serenity::model::event::GuildMemberAddEvent;
use serenity::model::guild::PartialGuild;
use serenity::model::id::{ChannelId, GuildId, MessageId, UserId};
use serenity::model::prelude::{
    Event, GuildChannel, Mentionable, Message, PermissionOverwrite, PermissionOverwriteType,
};
use serenity::model::Permissions;
use serenity::prelude::Context;
use serenity::utils::{Color, MessageBuilder};
use std::error::Error;
use std::iter::FromIterator;
use std::sync::Arc;

pub async fn on_event(ctx: &Context, event: &Event, data: Arc<BotData>) {
    let event = match event {
        Event::MessageCreate(e) => e,
        _ => {
            return;
        }
    };
    let guild_id = match &event.message.guild_id {
        Some(guild) => guild,
        None => return,
    };
    let c = &match data.config.get(&guild_id) {
        Some(config) => config,
        None => {
            warn!("Unknown guild {}, ignoring", guild_id);
            return;
        }
    }
    .publish;

    if !c.contains(&event.message.channel_id) {
        return;
    }

    let response = match match data
        .client
        .post(&format!(
            "{}/v{}/channels/{}/messages/{}/crosspost",
            "https://discord.com/api", 8, event.message.channel_id, event.message.id
        ))
        .header(CONTENT_LENGTH, 0)
        .header(AUTHORIZATION, &ctx.http.token)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            error!(
                "Could not attempt to crosspost message {} : {}",
                event.message.id, e
            );
            return;
        }
    }
    .error_for_status()
    {
        Ok(resp) => resp,
        Err(e) => {
            error!("Could not crosspost message {} : {}", event.message.id, e);
            return;
        }
    };
}
