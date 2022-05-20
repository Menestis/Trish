use serenity::framework::standard::macros::group;
use serenity::client::Context;
use serenity::model::prelude::{Message, Mention};
use serenity::framework::standard::{Args, CommandResult, macros::command};
use crate::discord::data::DataKey;
use serenity::model::Permissions;
use log::*;
use serenity::model::id::UserId;
use serenity::model::misc::UserIdParseError;
use crate::database::queries;

#[group]
#[commands(mute, demute)]
pub struct Moderation;


#[command]
#[only_in("guilds")]
#[required_permissions("MANAGE_MESSAGES")]
pub async fn mute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(guild) => { guild }
        None => {
            warn!("Guild have been checked previously, but is now 'None'");
            return Ok(());
        }
    };

    let data = {
        let guard = ctx.data.read().await;
        guard.get::<DataKey>().expect("Data").clone()
    };

    let role = match match data.config.get(&guild_id) {
        None => {
            msg.reply_ping(&ctx, "Ce serveur n'est pas configuré pour fonctionner avec trish !").await?;
            return Ok(());
        }
        Some(c) => c.mute_role
    } {
        None => {
            msg.reply_ping(&ctx, "Ce serveur n'a pas configuré de role de mute !").await?;
            return Ok(());
        }
        Some(role) => role
    };

    let m = args.remains().unwrap_or("");

    let to_mute = match m.trim_start_matches("<@").trim_start_matches("!").trim_end_matches(">").parse::<UserId>() {
        Ok(id) => id,
        Err(e) => {
            msg.reply_ping(&ctx, "Argument invalide").await?;
            return Ok(())
        }
    };

    if let Err(e) = queries::mutes::create_mute(&data.db, &guild_id, &to_mute).await {
        error!("{}", e);
        msg.reply_ping(&ctx, "Une erreur est survenue").await?;
        return Ok(());
    }


    ctx.http.add_member_role(guild_id.0, to_mute.0, role.0, None).await?;


    msg.reply_ping(&ctx, "Cet utilisateur à été réduit au silence !").await?;


    Ok(())
}

#[command]
#[only_in("guilds")]
#[required_permissions("MANAGE_MESSAGES")]
pub async fn demute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(guild) => { guild }
        None => {
            warn!("Guild have been checked previously, but is now 'None'");
            return Ok(());
        }
    };

    let m = args.message();

    let to_demute = match m.trim_start_matches("<@").trim_start_matches("!").trim_end_matches(">").parse::<UserId>() {
        Ok(id) => id,
        Err(e) => {
            msg.reply(ctx, "Argument invalide").await?;
            return Ok(())
        }
    };

    let data = {
        let guard = ctx.data.read().await;
        guard.get::<DataKey>().expect("Data").clone()
    };

    let role = match match data.config.get(&guild_id) {
        None => {
            msg.reply_ping(&ctx, "Ce serveur n'est pas configuré pour fonctionner avec trish !").await?;
            return Ok(());
        }
        Some(c) => c.mute_role
    } {
        None => {
            msg.reply_ping(&ctx, "Ce serveur n'a pas configuré de role de mute !").await?;
            return Ok(());
        }
        Some(role) => role
    };

    if let Err(e) = queries::mutes::delete_mute(&data.db, &guild_id, &to_demute).await {
        error!("{}", e);
        msg.reply_ping(&ctx, "Une erreur est survenue").await?;
        return Ok(());
    }

    msg.reply_ping(&ctx, "Cet utilisateur peut de nouveau parler !").await?;

    ctx.http.remove_member_role(guild_id.0, to_demute.0, role.0, None).await?;

    Ok(())
}