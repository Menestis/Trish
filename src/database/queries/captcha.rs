use serenity::model::prelude::{GuildId, UserId, ChannelId, MessageId};
use crate::database::DoloredDatabase;
use sqlx::query;

#[derive(Debug)]
pub struct DbCaptcha {
    pub guild: GuildId,
    pub user: UserId,
    pub channel: ChannelId,
    pub message: MessageId,
    pub captcha: String,
}

pub async fn create_captcha(db: &DoloredDatabase, captcha: &DbCaptcha) -> Result<(), sqlx::Error> {
    query!("INSERT INTO captchas (guild, user, channel, message, captcha) VALUE (?, ?, ?, ?, ?);", captcha.guild.0, captcha.user.0, captcha.channel.0, captcha.message.0, captcha.captcha)
        .execute(db).await?;

    Ok(())
}

pub async fn update_captcha(db: &DoloredDatabase, guild: &GuildId, user: &UserId, new_captcha: &str, new_message: MessageId) -> Result<(), sqlx::Error> {
    query!("UPDATE captchas SET captcha = ?, message = ? WHERE guild = ? AND user = ?;",new_captcha, new_message.0, guild.0, user.0)
        .execute(db).await?;
    Ok(())
}

pub async fn delete_captcha(db: &DoloredDatabase, guild: &GuildId, user: &UserId) -> Result<(), sqlx::Error> {
    query!("DELETE FROM captchas WHERE guild = ? AND user = ?;", guild.0, user.0)
        .execute(db).await?;
    Ok(())
}

pub async fn get_captcha(db: &DoloredDatabase, guild: &GuildId, user: &UserId) -> Result<Option<DbCaptcha>, sqlx::Error> {
    Ok(query!("SELECT user, guild, channel, message, captcha FROM captchas WHERE guild = ? AND user = ?;", guild.0, user.0)
        .fetch_optional(db).await?.map(|adhoc| DbCaptcha {
        guild: GuildId(adhoc.guild),
        user: UserId(adhoc.user),
        channel: ChannelId(adhoc.channel),
        message: MessageId(adhoc.message),
        captcha: adhoc.captcha,
    }))
}

pub async fn get_captcha_by_channel(db: &DoloredDatabase, guild: &GuildId, channel: &ChannelId) -> Result<Option<DbCaptcha>, sqlx::Error> {
    Ok(query!("SELECT user, guild, channel, message, captcha FROM captchas WHERE guild = ? AND channel = ?;", guild.0, channel.0)
        .fetch_optional(db).await?.map(|adhoc| DbCaptcha {
        guild: GuildId(adhoc.guild),
        user: UserId(adhoc.user),
        channel: ChannelId(adhoc.channel),
        message: MessageId(adhoc.message),
        captcha: adhoc.captcha,
    }))
}