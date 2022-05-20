use serenity::model::prelude::{GuildId, UserId, ChannelId, MessageId};
use crate::database::DoloredDatabase;
use sqlx::query;

pub async fn create_mute(db: &DoloredDatabase, guild: &GuildId,user: &UserId) -> Result<(), sqlx::Error> {
    query!("INSERT INTO mutes (guild, user) VALUE (?, ?);", guild.0, user.0)
        .execute(db).await?;

    Ok(())
}

pub async fn delete_mute(db: &DoloredDatabase, guild: &GuildId, user: &UserId) -> Result<(), sqlx::Error> {
    query!("DELETE FROM mutes WHERE guild = ? AND user = ?;", guild.0, user.0)
        .execute(db).await?;
    Ok(())
}

pub async fn is_muted(db: &DoloredDatabase, guild: &GuildId, user: &UserId) -> Result<bool, sqlx::Error> {
    Ok(query!("SELECT * FROM mutes WHERE guild = ? AND user = ?;", guild.0, user.0)
        .fetch_optional(db).await?.is_some())
}