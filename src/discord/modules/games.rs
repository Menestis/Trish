use serenity::prelude::{Context, Mentionable};
use serenity::model::prelude::{Event, PermissionOverwrite, GuildChannel, PermissionOverwriteType, Message, User, Member, MessageDeleteEvent};
use std::sync::Arc;
use serenity::model::id::{GuildId, UserId, ChannelId, MessageId};
use serenity::model::event::{GuildMemberAddEvent, GuildMemberRemoveEvent, GuildMemberUpdateEvent, ReactionAddEvent, MessageCreateEvent};
use std::error::Error;
use log::*;
use serenity::model::Permissions;
use serenity::model::guild::PartialGuild;
use captcha::{gen, Difficulty};
use std::iter::FromIterator;
use serenity::utils::{MessageBuilder, Color, Colour};
use serenity::model::channel::{ReactionType, Channel, Reaction, EmbedField};
use crate::discord::data::BotData;
use anyhow::{Result, bail};

use crate::database::queries::games::DbGame;
use crate::database::queries::games as queries;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

pub async fn on_event(ctx: &Context, event: &Event, data: Arc<BotData>) {
    match match event {
        Event::MessageCreate(add) => new_message(ctx, add, &data).await,
        Event::MessageDelete(remove) => { message_deleted(ctx, remove, &data).await }
        Event::ReactionAdd(r) => { reaction(ctx, &r.reaction, true, &data).await }
        Event::ReactionRemove(r) => { reaction(ctx, &r.reaction, false, &data).await }
        _ => { Ok(()) }
    } {
        Ok(()) => {}
        Err(e) => {
            error!("An error occurred while processing event for module games : {}", e);
        }
    }
}

pub async fn new_message(ctx: &Context, event: &MessageCreateEvent, data: &BotData) -> Result<()> {
    let guild = match event.message.guild_id {
        None => {
            return Ok(());
        }
        Some(guild) => guild
    };
    let cs = &match data.config.get(&guild) {
        Some(config) => { config }
        None => {
            warn!("Unknown guild {}, ignoring", guild);
            return Ok(());
        }
    }.games;


    for c in cs {
        if c.confirmation_channel.0 != event.message.channel_id.0 {
            continue
        }

        event.message.react(ctx, ReactionType::Unicode("✅".to_string())).await?;
        //event.message.react(ctx, ReactionType::Unicode("🛡️".to_string())).await?;
        break
    }

    return Ok(())
}

pub async fn message_deleted(ctx: &Context, event: &MessageDeleteEvent, data: &BotData) -> Result<()> {
    let guild = match event.guild_id {
        None => {
            return Ok(());
        }
        Some(guild) => guild
    };
    let cs = &match data.config.get(&guild) {
        Some(config) => { config }
        None => {
            warn!("Unknown guild {}, ignoring", guild);
            return Ok(());
        }
    }.games;

    for c in cs {
        if event.channel_id != c.publish_channel {
            continue
        }

        if let Some(game) = queries::get_game(&data.db, &guild, &c.publish_channel, &event.message_id).await? {
            game.channel.delete(ctx).await?;
            guild.delete_role(ctx, game.role).await?;
            queries::delete_game(&data.db, &guild, &game.announcement_channel, &game.announcement_msg).await?;
        }
        break
    }



    Ok(())
}

pub async fn reaction(ctx: &Context, reaction: &Reaction, added: bool, data: &BotData) -> Result<()> {
    let guild = match reaction.guild_id {
        None => {
            return Ok(());
        }
        Some(guild) => guild
    };

    let cs = &match data.config.get(&guild) {
        Some(config) => { config }
        None => {
            warn!("Unknown guild {}, ignoring", guild);
            return Ok(());
        }
    }.games;



    let user = reaction.user(ctx).await?;

    if user.bot {
        return Ok(());
    }



    let member = guild.member(ctx, user.id).await?;

    for c in cs {
        let has_role = member.roles.iter().any(|role| c.confirmation_roles.contains(role));

        if reaction.emoji.as_data().as_str() == "✅" && c.confirmation_channel == reaction.channel_id {
            if has_role {
                let reactions = reaction.channel_id.reaction_users(ctx, reaction.message_id, ReactionType::Unicode("✅".to_string()), None, None).await?;
                let validators: Vec<&User> = reactions.iter().filter(|&x| !x.bot).collect();
                if added && validators.len() <= 1 {
                    reaction.channel_id.create_reaction(ctx, &reaction.message_id, ReactionType::Unicode("🗒️".to_string())).await?;
                } else if !added && validators.is_empty() {
                    reaction.channel_id.delete_reaction_emoji(ctx, reaction.message_id, ReactionType::Unicode("🗒️".to_string())).await?;
                }
            } else {
                reaction.delete(ctx).await?;
            }
        } else if added && reaction.emoji.as_data().as_str() == "🗒️" && c.confirmation_channel == reaction.channel_id {
            let message = reaction.message(ctx).await?;

            if message.author.id != user.id && !has_role {
                reaction.delete(ctx).await?;
                return Ok(());
            }

            let dat = match parse_msg(&message.content) {
                Ok(map) => map,
                Err(e) => {
                    reaction.delete(ctx).await?;
                    return Ok(());
                }
            };

            let everyone_role = match guild.to_guild_cached(ctx) {
                Some(guild) => { guild.role_by_name("@everyone").expect("Everyone").clone() }
                None => {
                    warn!("Guild {} not in cache, fetching", guild);
                    let p: PartialGuild = guild.to_partial_guild(ctx).await?;
                    p.role_by_name("@everyone").expect("Everyone").clone()
                }
            };

            let role = guild.create_role(ctx, |r| {
                r.name(dat.get("Nom").unwrap().replace(" ", "-"));
                r.permissions(Permissions::empty());
                r
            }).await?.id;

            let mut perms = vec![
                PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::VIEW_CHANNEL,
                    kind: PermissionOverwriteType::Role(everyone_role.id),
                },
                PermissionOverwrite {
                    allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::MANAGE_MESSAGES | Permissions::MENTION_EVERYONE,
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Member(message.author.id),
                },
                PermissionOverwrite {
                    allow: Permissions::VIEW_CHANNEL,
                    deny: Permissions::SEND_MESSAGES,
                    kind: PermissionOverwriteType::Role(role),
                },
            ];

            for x in &c.confirmation_roles {
                perms.push(PermissionOverwrite {
                    allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::MANAGE_MESSAGES | Permissions::MENTION_EVERYONE,
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Role(*x),
                });
            }


            //let reactions = reaction.channel_id.reaction_users(ctx, reaction.message_id, ReactionType::Unicode("🛡️".to_string()), None, None).await?;
            // let moderators: Vec<&User> = reactions.iter().filter(|&x| !x.bot).collect();
            // for x in &moderators {
            //     perms.push(PermissionOverwrite {
            //         allow: Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES | Permissions::MANAGE_MESSAGES | Permissions::MENTION_EVERYONE,
            //         deny: Permissions::empty(),
            //         kind: PermissionOverwriteType::Member(x.id),
            //     });
            // }

            let game_chann = guild.create_channel(ctx, |cc| {
                cc.name(dat.get("Nom").unwrap());
                cc.topic(dat.get("Date").unwrap());
                cc.category(c.game_category);
                cc.permissions(perms);
                cc
            }).await?;

            let announcement_msg = c.publish_channel.send_message(ctx, |cm| {
                cm.content(c.publish_mention.mention());
                cm.embed(|ce| {
                    ce.author(|author| {
                        author.name(&message.author.name);
                        match &message.author.avatar_url() {
                            None => {}
                            Some(url) => { author.icon_url(url); }
                        };
                        author
                    });

                    ce.colour(Colour::from_rgb(128, 0, 128));

                    ce.title(dat.get("Nom").unwrap());
                    let mut desc = format!("**Date: ** {}\n", dat.get("Date").unwrap());

                    for (k, v) in dat.iter() {
                        if k != "Nom" && k != "Date" && k != "" && v != ""{
                            desc.push_str(&format!("**{}: **{}\n", k, v));
                        }
                    }

                    ce.description(desc);


                    // for (k, v) in dat.iter() {
                    //     if k != "Nom" && k != "Date" {
                    //         ce.field(k, v, v.len() < 40);
                    //     }
                    // }

                    // if !moderators.is_empty() {
                    //     let mentions: String = moderators.iter().map(|x| x.mention().to_string()).collect::<Vec<String>>().join(" ");
                    //     let mentions_len = mentions.len();
                    //     ce.field(if moderators.len() > 1 {
                    //         "Modérateurs"
                    //     } else {
                    //         "Modérateur"
                    //     }, mentions, mentions_len < 60);
                    // }

                    ce.footer(|f| {
                        f.text("Réagissez avec ✅ pour participer !")
                    });
                    ce
                });

                cm.reactions(vec![ReactionType::Unicode("✅".to_string())]);
                cm
            }).await?.id;


            let game = queries::DbGame {
                guild,
                announcement_channel: c.publish_channel,
                announcement_msg,
                channel: game_chann.id,
                role,
            };

            queries::create_game(&data.db, &game).await?;

            reaction.channel_id.delete_message(ctx, reaction.message_id).await?;
        } else if reaction.emoji.as_data().as_str() == "✅" && c.publish_channel == reaction.channel_id && reaction.user_id.is_some() {
            let game = match queries::get_game(&data.db, &guild, &c.publish_channel, &reaction.message_id).await? {
                None => continue,
                Some(game) => game
            };
            if added {
                ctx.http.add_member_role(guild.0, user.id.0, game.role.0, None).await?;
            } else {
                ctx.http.remove_member_role(guild.0, user.id.0, game.role.0, None).await?;
            }
        }

    }

    Ok(())
}

fn parse_msg(s: &str) -> Result<HashMap<String, String>> {
    let mut data = HashMap::new();

    let split = s.split("\n");

    for line in split {
        if line.is_empty() {
            continue;
        }
        let mut vec = line.split(":").collect::<Vec<&str>>();
        let k = match vec.first() {
            None => bail!("No key value pair"),
            Some(&k) => k.trim()
        };
        let vv: Vec<&str> = vec.drain(1..).collect();
        data.insert(k.to_string(), vv.join(":").trim().to_string());
    }

    if data.contains_key("Nom") && data.contains_key("Date") {
        return Ok(data);
    } else {
        bail!("Manque donnée")
    }
}