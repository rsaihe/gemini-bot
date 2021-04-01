use crate::{colours, utils};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[group]
#[commands(avatar)]
struct General;

#[command]
#[aliases("pfp")]
#[description("Get a user's avatar.")]
#[usage("[user]")]
async fn avatar(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Try to determine the intended user.
    let (name, avatar) = match args.message() {
        "" => (msg.author.name.clone(), msg.author.face()),
        value => {
            let guild = msg.guild(&ctx.cache).await;
            match utils::parse_user_id(value, guild.as_ref()).await {
                Some(id) => {
                    let user = id.to_user(&ctx.http).await?;
                    (user.name.clone(), user.face())
                }
                None => {
                    msg.channel_id.say(&ctx, "User not found.").await?;
                    return Ok(());
                }
            }
        }
    };

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| e.colour(colours::SUCCESS).title(&name).image(&avatar))
        })
        .await?;

    Ok(())
}
