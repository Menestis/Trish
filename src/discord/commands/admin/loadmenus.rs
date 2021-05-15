use serenity::framework::standard::macros::command;
use serenity::prelude::Context;
use serenity::model::prelude::{Message, ReactionType};
use serenity::framework::standard::{Args, CommandResult};
use crate::discord::data::DataKey;
use serenity::model::id::EmojiId;
use log::*;

#[command]
#[only_in("guilds")]
#[required_permissions("ADMINISTRATOR")]
pub async fn loadmenus(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = {
        let guard = ctx.data.read().await;
        guard.get::<DataKey>().expect("Data").clone()
    };
    let guild_id = match msg.guild_id {
        Some(guild) => { guild }
        None => {
            warn!("Guild have been checked previously, but is now 'None'");
            return Ok(());
        }
    };
    let config = &match data.config.get(&guild_id) {
        None => {
            warn!("Guild have been checked previously, but is now not present in configuration");
            return Ok(());
        }
        Some(config) => config
    }.roles_menus;

    for menu in config {
        let menu_msg = menu.channel.message(ctx, menu.message).await?;
        let present: Vec<String> = menu_msg.reactions.iter().map(|reac| reac.reaction_type.as_data()).collect();
        for (reac, role) in menu.reactions.iter().filter(|(reac, data)| !present.contains(reac)) {
            let reaction_type = if reac.contains(":") {
                let split: Vec<&str> = reac.split(":").collect();
                let id = split.get(1).expect("Split").replace(":", "");
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(id.parse()?),
                    name: Some(split.get(1).expect("Split").to_string())
                }
            } else {
                ReactionType::Unicode(reac.clone())
            };
            menu_msg.react(&ctx, reaction_type).await?;
        }

    }

    msg.delete(ctx).await?;
    msg.channel_id.say(ctx, "Ok !").await?;
    Ok(())
}