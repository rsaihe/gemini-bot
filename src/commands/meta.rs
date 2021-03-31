use std::time::Instant;

use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[group]
#[commands(ping)]
pub struct Meta;

#[command]
#[description("Ping the bot.")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = Instant::now();

    let mut ping = msg.channel_id.say(&ctx.http, "Ping?").await?;

    let duration = start.elapsed().as_millis();

    ping.edit(&ctx, |m| m.content(format!("Pong! (took {} ms)", duration)))
        .await?;

    Ok(())
}
