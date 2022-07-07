use serenity::prelude::{Context, RawEventHandler};
use serenity::model::prelude::Event;
use futures::join;
use serenity::async_trait;
use crate::discord::data::DataKey;
use log::*;

pub struct Handler;

use super::modules::*;

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, ctx: Context, event: Event) {
        dispatch(ctx, event).await;
    }
}

pub async fn dispatch(ctx: Context, event: Event) {

    let data = {
        let guard = ctx.data.read().await;
        guard.get::<DataKey>().expect("Data").clone()
    };

    join!(
        captcha::on_event(&ctx, &event, data.clone()),
        logs::on_event(&ctx, &event, data.clone()),
        publish::on_event(&ctx, &event, data.clone()),
        rolemenus::on_event(&ctx, &event, data.clone()),
        games::on_event(&ctx, &event, data.clone()),
        kicklimits::on_event(&ctx, &event, data.clone()),
        link::on_event(&ctx, &event, data.clone())
    );
}