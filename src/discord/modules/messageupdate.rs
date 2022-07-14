use std::borrow::Borrow;
use std::sync::Arc;
use log::{debug, error, warn};
use serenity::client::Context;
use serenity::model::channel::AttachmentType;
use serenity::model::event::{Event, MessageCreateEvent, MessageUpdateEvent};
use serenity::utils::Color;
use crate::discord::data::BotData;
use anyhow::Result;

pub async fn on_event(ctx: &Context, event: &Event, data: Arc<BotData>) {
    match match event {
        Event::MessageCreate(create) => new_message(ctx, create, &data).await,
        Event::MessageUpdate(update) => { update_message(ctx, update, &data).await }
        _ => { Ok(()) }
    } {
        Ok(()) => {}
        Err(e) => {
            error!("An error occurred while processing event for module captcha : {}", e);
        }
    }
}

pub async fn update_message(ctx: &Context, event: &MessageUpdateEvent, data: &BotData) -> Result<()> {
    let guild_id = &match event.guild_id {
        Some(id) => { id }
        None => return Ok(())
    };

    let guard = data.last_messages.read().await;
    if guard.0.contains(&event.id){

        let c = match match data.config.get(guild_id) {
            Some(config) => { config }
            None => {
                warn!("Unknown guild {}, ignoring", guild_id);
                return Ok(());
            }
        }.message_logs.as_ref() {
            Some(c) => c,
            None => {
                debug!("No logs configured");
                return Ok(());
            }
        };

        c.channel.send_message(ctx, |m| {
            m.embed(|e| {
                e.color(Color::from_rgb(0, 0, 0));
                e.title(format!("| {}#{}", event.author.as_ref().map(|u| u.name.clone()).unwrap_or("Pseudonyme introuvable".to_string()), event.author.as_ref().map(|u| u.discriminator).unwrap_or_default()));
                e.description(format!("Old message : {} | New message : {}", guard.1.get(&event.id).unwrap_or(&"pas de message trouvé".to_string()), event.content.as_ref().unwrap_or(&"pas de message trouvé".to_string())))
            });
            m
        }).await?;
    }

    Ok(())
}

pub async fn new_message(ctx: &Context, event: &MessageCreateEvent, data: &BotData) -> Result<()> {
    if event.message.content.is_empty(){
        return Ok(())
    }

    let mut guard = data.last_messages.write().await;
    if let Some(toRm) = guard.0.push_back(event.message.id){
        guard.1.remove(&toRm);
    }

    guard.1.insert(event.message.id, event.message.content.clone());
    Ok(())
}