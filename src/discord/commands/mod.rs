use serenity::prelude::Context;
use serenity::model::prelude::{Message, UserId};
use serenity::framework::standard::{CommandResult, Args, HelpOptions, CommandGroup, help_commands, CommandOptions, Reason};
use log::*;
use std::collections::HashSet;
use serenity::framework::standard::macros::*;
use crate::discord::data::DataKey;
use uuid::Uuid;

mod admin;
pub use admin::*;

#[group]
#[commands(ping)]
pub struct General;


#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Pong !").await?;

    Ok(())
}

#[hook]
pub async fn after(ctx: &Context, msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => debug!("Processed command '{}'", command_name),
        Err(e) => {
            let uuid = Uuid::new_v4();
            error!("[{}] Command '{}' returned error {:?}", uuid, command_name, e);
            if let Err(e) = msg.reply_mention(ctx, format!("Une erreure est survenue, contactez un administrateur avec ce code: `{}`", uuid)).await{
                error!("Can't send error message to user");
            }
        }
    }
}


#[help]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
#[strikethrough_commands_tip_in_guild = "Les commandes ~~barrées~~ ne sont pas utilisables dans ce salon"]
#[strikethrough_commands_tip_in_dm = "Les commandes ~~barrées~~ ne sont pas utilisables dans ce salon"]
#[no_help_available_text = "Pas d'aide disponible pour cette commande"]
#[individual_command_tip = "Vous pouvez taper $help [commande] pour obtenir de l'aide sur cette commande"]
pub async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[check]
#[name = "channel"]
#[display_in_help(false)]
pub async fn channel_check(ctx: &Context, msg: &Message, args: &mut Args, opts: &CommandOptions) -> Result<(), Reason> {
    let data = {
        let guard = ctx.data.read().await;
        guard.get::<DataKey>().expect("Data").clone()
    };

    if msg.is_private() {
        return Ok(());
    }


    let guild_id = match msg.guild_id {
        Some(guild) => { guild }
        None => { return Ok(()); }
    };


    let allowed_channels = &match data.config.get(&guild_id) {
        Some(config) => { config }
        None => {
            warn!("Unknown guild {}, ignoring channel check", guild_id);
            return Err(Reason::Log("Wrong guild".to_string()));
        }
    }.allowed_channels;

    if allowed_channels.contains(&msg.channel_id) {
        Ok(())
    } else {
        Err(Reason::Log("Wrong channel".to_string()))
    }
}
