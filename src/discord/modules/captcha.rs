use serenity::prelude::Context;
use serenity::model::prelude::{Event, PermissionOverwrite, GuildChannel, PermissionOverwriteType, Message, User, Member};
use std::sync::Arc;
use serenity::model::id::{GuildId, UserId, ChannelId, MessageId};
use serenity::model::event::{GuildMemberAddEvent, GuildMemberRemoveEvent, GuildMemberUpdateEvent, ReactionAddEvent, MessageCreateEvent};
use std::error::Error;
use log::*;
use serenity::model::Permissions;
use serenity::model::guild::PartialGuild;
use captcha::{gen, Difficulty};
use std::iter::FromIterator;
use serenity::utils::{MessageBuilder, Color};
use serenity::http::AttachmentType;
use serenity::model::channel::{ReactionType, Channel};
use crate::discord::data::BotData;
use anyhow::{Result, bail};

use crate::database::queries::captcha::DbCaptcha;
use crate::database::queries::captcha as queries;
use serenity::builder::CreateEmbed;

fn gen_captcha() -> Option<(Vec<u8>, String)> {
    let captcha = gen(Difficulty::Medium);
    let (data, chars) = match captcha.as_png() {
        Some(data) => (data, String::from_iter(captcha.chars())),
        None => return None,
    };
    Some((data, chars.to_lowercase()))
}

pub async fn on_event(ctx: &Context, event: &Event, data: Arc<BotData>) {
    match match event {
        Event::GuildMemberAdd(add) => new_captcha(ctx, add, &data).await,
        Event::GuildMemberRemove(remove) => { delete_captcha(ctx, remove, &data).await }
        Event::ReactionAdd(reaction) => { check_refresh(ctx, reaction, &data).await }
        Event::MessageCreate(message) => { check_validation(ctx, message, &data).await }
        _ => { Ok(()) }
    } {
        Ok(()) => {}
        Err(e) => {
            error!("An error occurred while processing event for module captcha : {}", e);
        }
    }
}

pub async fn new_captcha(ctx: &Context, event: &GuildMemberAddEvent, data: &BotData) -> Result<()> {
    let guild_id = &event.guild_id;
    let c = match match data.config.get(guild_id) {
        Some(config) => { config }
        None => {
            warn!("Unknown guild {}, ignoring", guild_id);
            return Ok(());
        }
    }.captcha.as_ref() {
        Some(c) => { c }
        None => {
            {
                debug!("No captcha configured for {}", guild_id);
                return Ok(());
            }
        }
    };
    let db = &data.db;

    debug!("Creating new captcha");

    let everyone_role = match guild_id.to_guild_cached(ctx).await {
        Some(guild) => { guild.role_by_name("@everyone").expect("Everyone").clone() }
        None => {
            warn!("Guild {} not in cache, fetching", guild_id);
            let p: PartialGuild = guild_id.to_partial_guild(ctx).await?;
            p.role_by_name("@everyone").expect("Everyone").clone()
        }
    };

    let mut member = event.member.clone();

    //On commence par lui add son role
    member.add_role(ctx, c.rank).await?;

    //On créé une nouvelle chaine de captcha
    let channel: ChannelId = guild_id.create_channel(ctx, |cr| {
        cr.name(&c.channel_name);
        cr.category(&c.category);
        cr.permissions(vec![
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::CONNECT | Permissions::READ_MESSAGES,
                kind: PermissionOverwriteType::Role(everyone_role.id),
            },
            PermissionOverwrite {
                allow: Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(member.user.id),
            }])
    }).await?.id;

    //On génère un captcha
    let (data, code) = match gen_captcha() {
        Some(cap) => { cap }
        None => bail!("Could not generate captcha")
    };

    let message: Message = channel.send_message(ctx, |m| {
        m.content(MessageBuilder::new().mention(&member).push("\n").push(&c.message).build());
        m.add_file(AttachmentType::from((data.as_slice(), "captcha.png")));
        m.reactions(vec![ReactionType::Unicode("🔄".to_string())])
    }).await?;

    let captcha = DbCaptcha {
        guild: guild_id.clone(),
        user: member.user.id,
        channel,
        message: message.id,
        captcha: code,
    };

    queries::create_captcha(db, &captcha).await?;

    Ok(())
}

pub async fn delete_captcha(ctx: &Context, event: &GuildMemberRemoveEvent, data: &BotData) -> Result<()> {
    let guild_id = &event.guild_id;
    let c = match match data.config.get(guild_id) {
        Some(config) => { config }
        None => {
            warn!("Unknown guild {}, ignoring", guild_id);
            return Ok(());
        }
    }.captcha.as_ref() {
        Some(c) => { c }
        None => {
            {
                debug!("No captcha configured for {}", guild_id);
                return Ok(());
            }
        }
    };
    let db = &data.db;

    let captcha = match queries::get_captcha(db, guild_id, &event.user.id).await? {
        Some(captcha) => { captcha }
        None => return Ok(())
    };

    captcha.channel.delete(ctx).await?;

    queries::delete_captcha(db, guild_id, &event.user.id).await?;

    Ok(())
}

pub async fn check_refresh(ctx: &Context, event: &ReactionAddEvent, data: &BotData) -> Result<()> {
    if event.reaction.emoji.to_string() != "🔄" {
        return Ok(());
    }
    if event.reaction.user(ctx).await?.bot {
        return Ok(());
    }
    let guild_id = match &event.reaction.guild_id {
        Some(guild) => { guild }
        None => return Ok(())
    };
    let c = match match data.config.get(guild_id) {
        Some(config) => { config }
        None => {
            warn!("Unknown guild {}, ignoring", guild_id);
            return Ok(());
        }
    }.captcha.as_ref() {
        Some(c) => { c }
        None => {
            {
                debug!("No captcha configured for {}", guild_id);
                return Ok(());
            }
        }
    };
    let db = &data.db;

    let mut captcha = match queries::get_captcha_by_channel(db, guild_id, &event.reaction.channel_id).await? {
        Some(captcha) if captcha.message.0 == event.reaction.message_id.0 => { captcha }
        _ => {
            return Ok(());
        }
    };


    ctx.http.delete_message(captcha.channel.0, captcha.message.0).await?;

    let (data, code) = match gen_captcha() {
        Some(k) => { k }
        None => {
            error!("Could not generate captcha");
            return Ok(());
        }
    };


    let new_message: Message = captcha.channel.send_message(ctx, |m| {
        m.content(MessageBuilder::new().mention(&captcha.user).push("\n").push(&c.message).build());
        m.add_file(AttachmentType::from((data.as_slice(), "captcha.png")));
        m.reactions(vec![ReactionType::Unicode("🔄".to_string())])
    }).await?;

    queries::update_captcha(db, &captcha.guild, &captcha.user, &code, new_message.id).await?;

    Ok(())
}

pub async fn check_validation(ctx: &Context, event: &MessageCreateEvent, data: &BotData) -> Result<()> {
    if event.message.author.bot {
        return Ok(());
    }
    let guild_id = match &event.message.guild_id {
        Some(guild) => { guild }
        None => return Ok(())
    };
    let c = match match data.config.get(guild_id) {
        Some(config) => { config }
        None => {
            warn!("Unknown guild {}, ignoring", guild_id);
            return Ok(());
        }
    }.captcha.as_ref() {
        Some(c) => { c }
        None => {
            {
                debug!("No captcha configured for {}", guild_id);
                return Ok(());
            }
        }
    };
    match event.message.channel_id.name(ctx).await { //Be a little bit smart, since cache has literally no cost
        Some(name) if name != c.channel_name => {
            return Ok(());
        }
        _ => {}
    }
    let db = &data.db;

    let mut captcha = match queries::get_captcha_by_channel(db, guild_id, &event.message.channel_id).await? {
        Some(captcha) => { captcha }
        _ => {
            return Ok(());
        }
    };

    if event.message.content.to_lowercase() == captcha.captcha {
        let mut member: Member = guild_id.member(ctx, captcha.user).await?;
        member.add_role(ctx, c.after_rank).await?;
        member.remove_role(ctx, c.rank).await?;
        captcha.channel.delete(ctx).await?;

        if let Some(log) = c.log_channel {
            log.send_message(ctx, |m| {
                m.embed(|e| {
                    e.color(Color::DARK_GREEN);
                    e.title(format!("+ {}#{}", member.user.name, member.user.discriminator));
                    e
                });
                m
            }).await?;
        }

        queries::delete_captcha(db, &captcha.guild, &captcha.user).await?;

        match captcha.user.create_dm_channel(ctx).await {
            Ok(dm) => {
                dm.say(ctx, &c.welcome_message).await?;
            }
            Err(e) => {
                warn!("Could not create dm channel : {}", e);
            }
        }
    } else {
        captcha.channel.say(ctx, &c.fail_message).await?;
    }


    Ok(())
}
