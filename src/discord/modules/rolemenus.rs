use serenity::prelude::Context;
use serenity::model::prelude::{Event, PermissionOverwrite, GuildChannel, PermissionOverwriteType, Message, User};
use std::sync::Arc;
use serenity::model::id::{GuildId, UserId, ChannelId, MessageId};
use serenity::model::event::{GuildMemberAddEvent, GuildMemberRemoveEvent, GuildMemberUpdateEvent, ReactionAddEvent, MessageCreateEvent, ReadyEvent};
use std::error::Error;
use log::*;
use serenity::model::Permissions;
use serenity::model::guild::PartialGuild;
use captcha::{gen, Difficulty};
use std::iter::FromIterator;
use serenity::utils::{MessageBuilder, Color};
use serenity::model::channel::{ReactionType, Channel, Reaction};
use crate::discord::data::BotData;


pub async fn on_event(ctx: &Context, event: &Event, data: Arc<BotData>) {
    match match event {
        Event::ReactionAdd(event) => { reaction(ctx, &event.reaction, &data, true).await }
        Event::ReactionRemove(event) => { reaction(ctx, &event.reaction, &data, false).await }
        _ => { Ok(()) }
    } {
        Ok(()) => {}
        Err(e) => {
            error!("An error occurred while processing event for role menu : {}", e);
        }
    }
}

pub async fn reaction(ctx: &Context, reac: &Reaction, data: &BotData, added: bool) -> Result<(), Box<dyn Error>> {
    let user: User = reac.user(ctx).await?;
    if user.bot {
        return Ok(());
    }
    let guild_id = &match reac.guild_id {
        Some(id) => { id }
        None => return Ok(())
    };
    let c = &match data.config.get(guild_id) {
        Some(config) => { config }
        None => {
            warn!("Unknown guild {}, ignoring", guild_id);
            return Ok(());
        }
    }.roles_menus;

    let r = &reac.emoji.as_data();

    let (inverted, role) = match c.iter().find(|&menu| menu.channel.0 == reac.channel_id.0 && menu.message.0 == reac.message_id.0 && menu.reactions.contains_key(r)) {
        Some(menu) => (menu.inverted, menu.reactions.get(r).unwrap()),
        None => return Ok(())
    };

    if inverted {
        if added {
            ctx.http.remove_member_role(guild_id.0, user.id.0, role.0, Some("RoleMenu")).await?;
        } else {
            ctx.http.add_member_role(guild_id.0, user.id.0, role.0, Some("RoleMenu")).await?;
        }
    } else {
        if added {
            ctx.http.add_member_role(guild_id.0, user.id.0, role.0, Some("RoleMenu")).await?;
        } else {
            ctx.http.remove_member_role(guild_id.0, user.id.0, role.0, Some("RoleMenu")).await?;
        }
    }


    Ok(())
}
