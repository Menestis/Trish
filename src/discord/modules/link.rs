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
use serde_json::Value;
use serenity::model::interactions::application_command::{ApplicationCommandInteractionDataOption, ApplicationCommandOptionType};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use skynet_api::apis::discord_api;
use skynet_api::apis::discord_api::{ApiDiscordLinkCodePostError, ApiDiscordLinkDiscordDeleteError};

pub async fn on_event(ctx: &Context, event: &Event, data: Arc<BotData>) {
    let interaction_create = match event {
        Event::Ready(e) => {
            for (id, c) in &data.config {
                if c.can_link {
                    if let Err(e) = id.set_application_commands(&ctx, |f| {
                        f.create_application_command(|cmd| cmd.name("link").description("Liez votre jeu à notre discord")
                            .create_option(|ca| ca.name("code").description("code").description("Code obtenu en jeu").required(true).kind(ApplicationCommandOptionType::String)))
                    }).await {
                        error!("Could not create application command {}", e)
                    }
                }
            }
            return;
        }
        Event::InteractionCreate(e) => e,
        Event::GuildMemberRemove(remove) => {
            if let Some(c) = data.config.get(&remove.guild_id) {
                if c.can_link {
                    if let Err(e) = discord_api::api_discord_link_discord_delete(&data.skynet, &remove.user.id.to_string()).await {
                        error!("{}",e)
                    }
                };
            };
            return;
        }
        _ => {
            return;
        }
    };


    let command = match &interaction_create.interaction {
        Interaction::ApplicationCommand(command) => command,
        _ => return
    };

    if command.data.name != "link" {
        return;
    }

    let guild_id = match command.data.guild_id {
        None => {
            return;
        }
        Some(guild) => guild
    };

    let c = match data.config.get(&guild_id) {
        Some(config) => { config }
        None => {
            warn!("Unknown guild {}, ignoring", guild_id);
            return;
        }
    };
    if !c.allowed_channels.contains(&command.channel_id) {
        if let Err(e) = command.create_interaction_response(&ctx, |cr| cr.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|cd| cd.ephemeral(true).embed(|f| f.color(Color::RED).title("Menestis | Erreur").description("Merci d'utiliser un salon bot pour utiliser cette commande.")))).await {
            error!("{}", e);
        }
        return;
    }


    let idata = match command.data.options.get(0) {
        None => {
            if let Err(e) = command.create_interaction_response(&ctx, |cr| cr.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|cd| cd.ephemeral(true).embed(|f| f.color(Color::RED).title("Menestis | Erreur").description("Merci de spécifier un code \n\n**»** `/link` en jeu pour en obtenir un")))).await {
                error!("{}", e);
            }
            return;
        }
        Some(data) => data
    };

    let code = if let Value::String(code) = idata.value.as_ref().unwrap() {
        code
    } else {
        warn!("Link interaction without a valid value");
        return;
    };

    match discord_api::api_discord_link_code_post(&data.skynet, &code, &command.user.id.to_string()).await {
        Ok(()) => {
            if let Err(e) = command.create_interaction_response(&ctx, |cr| cr.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|cd| cd.ephemeral(true).embed(|f| f.color(Color::BLUE).title("Menestis | Succés").description("Votre compte discord est maintenant lié à votre compte Minecraft !\n\nBon jeu sur Menestis !")))).await {
                error!("{}", e);
            }
        }
        Err(e) => {
            match &e {
                skynet_api::apis::Error::ResponseError(resp) => {
                    if resp.status.as_u16() == 404 {
                        if let Err(e) = command.create_interaction_response(&ctx, |cr| cr.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|cd| cd.ephemeral(true).embed(|f| f.color(Color::BLUE).title("Menestis | Erreur").description("Merci de spécifier un code valide\n\n**»** `/link` en jeu pour en obtenir un")))).await {}
                    } else {
                        error!("Skynet : {}", e);
                    }
                }
                _ => error!("Skynet : {}", e)
            }
        }
    };
}

