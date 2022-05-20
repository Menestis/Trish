use serenity::prelude::Context;
use serenity::model::prelude::{Event, PermissionOverwrite, GuildChannel, PermissionOverwriteType, Message, User, Member};
use std::sync::Arc;
use serenity::model::id::{GuildId, UserId, ChannelId, MessageId};
use serenity::model::event::{GuildMemberAddEvent, GuildMemberRemoveEvent, GuildMemberUpdateEvent, ReactionAddEvent, MessageCreateEvent};
use std::error::Error;
use log::*;
use serenity::model::Permissions;
use serenity::model::guild::PartialGuild;
use captcha::{gen, Difficulty, Captcha, Geometry};
use std::iter::FromIterator;
use serenity::utils::{MessageBuilder, Color};
use serenity::model::channel::{ReactionType, Channel};
use crate::discord::data::BotData;
use anyhow::{Result, bail};

use crate::database::queries::captcha::DbCaptcha;
use crate::database::queries::captcha as queries;
use serenity::builder::CreateEmbed;
use captcha::filters::{Noise, Wave, Cow};


pub async fn on_event(ctx: &Context, event: &Event, data: Arc<BotData>) {
    // let msg_event = match event {
    //     Event::MessageCreate(e) => e,
    //     _ => { return; }
    // };
    //
    //
    // let msg = &msg_event.message;
    // let content = &msg.content.to_lowercase();
    // if content.contains(" immoral") || content.contains(" vol") || (content.contains(" appartiens") || content.contains(" apartiens")) {
    //     match msg.reply_ping(ctx, "Ceci vous aidera sûrement à comprendre : https://www.minecraft.net/fr-ca/eula").await {
    //         Ok(_) => {}
    //         Err(e) => {
    //             error!("{}", e);
    //         }
    //     };
    // }
}