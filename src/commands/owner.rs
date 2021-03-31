use crate::ShardManagerContainer;

use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::error;

#[group]
#[commands(quit)]
#[owners_only]
pub struct Owner;

#[command]
#[description("Shut down the bot.")]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    match data.get::<ShardManagerContainer>() {
        Some(manager) => {
            msg.channel_id.say(&ctx, "Shutting down!").await?;
            manager.lock().await.shutdown_all().await;
        }
        None => {
            msg.channel_id
                .say(&ctx, "There was a problem shutting down.")
                .await?;
            error!("Problem getting shard manager");
        }
    }

    Ok(())
}
