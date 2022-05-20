use serenity::prelude::Context;
use serenity::model::prelude::{Event, PermissionOverwrite, GuildChannel, PermissionOverwriteType, Message, Mentionable};
use std::sync::Arc;
use serenity::model::id::{GuildId, UserId, ChannelId, MessageId};
use serenity::model::event::GuildMemberAddEvent;
use std::error::Error;
use log::*;
use serenity::model::Permissions;
use serenity::model::guild::PartialGuild;
use captcha::{gen, Difficulty};
use std::iter::FromIterator;
use serenity::utils::{MessageBuilder, Color};
use crate::discord::data::BotData;


pub async fn on_event(ctx: &Context, event: &Event, data: Arc<BotData>) {
    let (join, guild, id, name, discriminator, avatar) = match event {
        Event::GuildMemberAdd(add) => (true, add.member.guild_id, add.member.user.id, add.member.user.name.clone(), add.member.user.discriminator, add.member.user.avatar.clone()),
        Event::GuildMemberRemove(remove) => (false, remove.guild_id, remove.user.id, remove.user.name.clone(), remove.user.discriminator, remove.user.avatar.clone()),
        _ => { return; }
    };

    let c = match match data.config.get(&guild) {
        Some(config) => { config }
        None => {
            warn!("Unknown guild {}, ignoring", guild);
            return;
        }
    }.logs.as_ref() {
        Some(c) => { c }
        None => {
            {
                debug!("No logs configured");
                return;
            }
        }
    };

    match c.channel.send_message(ctx, |m| {
        m.embed(|e| {
            if join {
                e.color(Color::ORANGE);
                e.title(format!("* {}#{}", name, discriminator));
            } else {
                e.color(Color::RED);
                e.title(format!("- {}#{}", name, discriminator));
            }
            e
        });
        m
    }).await {
        Ok(m) => {}
        Err(e) => {
            error!("{}", e);
        }
    };
}

