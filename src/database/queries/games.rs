use serenity::model::prelude::{GuildId, UserId, ChannelId, MessageId};
use crate::database::DoloredDatabase;
use sqlx::query;
use serenity::model::id::RoleId;

#[derive(Debug)]
pub struct DbGame {
    pub guild: GuildId,
    pub announcement_channel: ChannelId,
    pub announcement_msg: MessageId,
    pub channel: ChannelId,
    pub role: RoleId,
}

pub async fn create_game(db: &DoloredDatabase, game: &DbGame) -> Result<(), sqlx::Error> {
    query!("INSERT INTO games (guild, announcement_channel, announcement_msg, channel, role) VALUE (?, ?, ?, ?, ?);", game.guild.0, game.announcement_channel.0, game.announcement_msg.0, game.channel.0, game.role.0)
        .execute(db).await?;

    Ok(())
}

pub async fn delete_game(db: &DoloredDatabase, guild: &GuildId, announcement_channel: &ChannelId, announcement_msg: &MessageId) -> Result<(), sqlx::Error> {
    query!("DELETE FROM games WHERE guild = ? AND announcement_channel = ? AND announcement_msg = ?;", guild.0, announcement_channel.0, announcement_msg.0)
        .execute(db).await?;
    Ok(())
}

pub async fn get_game(db: &DoloredDatabase, guild: &GuildId, announcement_channel: &ChannelId, announcement_msg: &MessageId) -> Result<Option<DbGame>, sqlx::Error> {
    Ok(query!("SELECT guild, announcement_channel, announcement_msg, channel, role FROM games WHERE guild = ? AND announcement_channel = ? AND announcement_msg = ?;", guild.0, announcement_channel.0, announcement_msg.0)
        .fetch_optional(db).await?.map(|adhoc| DbGame {
        guild: GuildId(adhoc.guild),
        announcement_channel: ChannelId(adhoc.announcement_channel),
        announcement_msg: MessageId(adhoc.announcement_msg),
        channel: ChannelId(adhoc.channel),
        role: RoleId(adhoc.role),
    }))
}
