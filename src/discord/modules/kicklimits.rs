use serenity::prelude::Context;
use serenity::model::event::{Event, GuildMemberRemoveEvent};
use std::sync::Arc;
use crate::discord::data::BotData;
use log::*;
use serenity::model::Permissions;
use chrono::{Duration, Utc, DateTime};
use serenity::model::guild::AuditLogEntry;
use serenity::utils::Color;
use std::collections::HashMap;
use serenity::model::id::UserId;

pub async fn on_event(ctx: &Context, event: &Event, data: Arc<BotData>) {
    match match event {
        Event::GuildMemberRemove(remove) => on_kick(ctx, remove, &data).await,
        _ => { Ok(()) }
    } {
        Ok(()) => {}
        Err(e) => {
            error!("An error occurred while processing event for module captcha : {}", e);
        }
    }
}

pub async fn on_kick(ctx: &Context, event: &GuildMemberRemoveEvent, data: &BotData) -> anyhow::Result<()> {
    let logs_kick = event.guild_id.audit_logs(ctx, Some(20), None, None, None).await?;
    let logs_ban = event.guild_id.audit_logs(ctx, Some(22), None, None, None).await?;

    let now = Utc::now();


    let oldest = match now.checked_sub_signed(Duration::minutes(5)) {
        None => {
            warn!("Could not substract five minutes");
            return Ok(());
        }
        Some(oldest) => oldest
    };

    let mut users = HashMap::new();

    for entry in logs_kick.entries {
        if entry.id.created_at().unix_timestamp() > oldest.timestamp() {
            *users.entry(entry.user_id).or_insert(0) += 1;
        }
    }


    for entry in logs_ban.entries {
        if entry.id.created_at().unix_timestamp() > oldest.timestamp() {
            *users.entry(entry.user_id).or_insert(0) += 1;
        }
    }


    for (usr, v) in users {
        info!("User: {} / vl: {}", usr.0, v);
        //We have a problem bro
        if v > 10 {
            let mut member = event.guild_id.member(ctx, usr).await?;
            info!("Flagged user: {}#{}", member.user.name, member.user.discriminator);

            let permissions = member.permissions(ctx)?;

            if permissions.contains(Permissions::ADMINISTRATOR) {
                continue;
            }

            let roles = member.roles.clone();
            member.remove_roles(ctx, roles.as_slice()).await?;

            let c = match match data.config.get(&event.guild_id) {
                Some(config) => { config }
                None => {
                    warn!("Unknown guild {}, ignoring", event.guild_id);
                    return Ok(());
                }
            }.logs.as_ref() {
                Some(c) => c,
                None => {
                    debug!("No logs configured");
                    return Ok(());
                }
            };

            c.channel.send_message(ctx, |m| {
                m.embed(|e| {
                    e.color(Color::from_rgb(0, 0, 0));
                    e.title(format!("| {}#{}", member.user.name, member.user.discriminator))
                });
                m
            }).await?;
        }
    }


    Ok(())
}
