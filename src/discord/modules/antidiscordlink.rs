use serenity::prelude::Context;
use serenity::model::prelude::{Event, PermissionOverwrite, GuildChannel, PermissionOverwriteType, Message, Mentionable};
use std::sync::Arc;
use serenity::model::id::{GuildId, UserId, ChannelId, MessageId};
use serenity::model::event::{GuildMemberAddEvent, MessageCreateEvent, MessageUpdateEvent};
use std::error::Error;
use log::*;
use serenity::model::Permissions;
use serenity::model::guild::PartialGuild;
use captcha::{gen, Difficulty};
use std::iter::FromIterator;
use serenity::utils::{MessageBuilder, Color};
use crate::discord::data::BotData;


pub async fn on_event(ctx: &Context, event: &Event, data: Arc<BotData>) {
    match match event {
        Event::MessageCreate(create) => new_message(ctx, create, &data).await,
        Event::MessageUpdate(update) => update_message(ctx, update, &data).await,
        _ => { Ok(()) }
    } {
        Ok(()) => {}
        Err(e) => {
            error!("An error occurred while processing event for module captcha : {}", e);
        }
    }
}

pub async fn new_message(ctx: &Context, event: &MessageCreateEvent, data: &BotData) {
    if event.message.content.is_empty(){
        return;
    }
    
    if event.message.content.contains("discord.gg") {
        event.message.delete(&ctx.http);


        let c = match match data.config.get(guild_id) {
            Some(config) => { config }
            None => {
                warn!("Unknown guild {}, ignoring", guild_id);
                return;
            }
        }.message_logs.as_ref() {
            Some(c) => c,
            None => {
                debug!("No logs configured");
                return;
            }
        };

        c.channel.send_message(ctx, |m| {
            m.embed(|e| {
                e.color(Color::from_rgb(0, 0, 0));
                e.title(format!("| {}#{}", event.message.author.name, event.message.author.discriminator));
                e.description(format!("Message supprimé : \n {}", event.message.content))
            })
        }).await?;
    }
}

pub async fn update_message(ctx: &Context, event: &MessageUpdateEvent, data: &BotData) {
    if event.content.is_empty(){
        return;
    }


    let message_content = match &event.content {
        Some(str) => {
            str
        } _ => {
            return;
        }
    };

    if message_content.contains("discord.gg") {
        event.channel_id.delete_message(&ctx.http, event.id);


        let c = match match data.config.get(guild_id) {
            Some(config) => { config }
            None => {
                warn!("Unknown guild {}, ignoring", guild_id);
                return;
            }
        }.message_logs.as_ref() {
            Some(c) => c,
            None => {
                debug!("No logs configured");
                return;
            }
        };

        c.channel.send_message(ctx, |m| {
            m.embed(|e| {
                e.color(Color::from_rgb(0, 0, 0));
                e.title(format!("| {}#{}", event.author.expect("Could not load user for update event.").name, event.author.expect("Invalid author for update event").discriminator));
                e.description(format!("Message modifié supprimé : \n {}", message_content))
            })
        }).await?;
    }
}
