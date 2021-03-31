use std::collections::HashSet;
use std::time::Instant;

use serenity::framework::standard::macros::{command, group, help};
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::error;

#[help]
#[embed_error_colour("RED")]
#[embed_success_colour("#4cbb17")]
#[individual_command_tip("For help with a specific command, pass its name as an argument.")]
#[lacking_ownership("nothing")]
#[lacking_role("nothing")]
#[max_levenshtein_distance(2)]
#[no_help_available_text("Command not found.")]
#[strikethrough_commands_tip_in_dm("")]
#[strikethrough_commands_tip_in_guild("")]
async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, options, groups, owners).await;
    Ok(())
}

#[group]
#[commands(about, invite, ping)]
pub struct Meta;

#[command]
#[description("Display the bot's about page.")]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    // Get the current user.
    let user = ctx.cache.current_user().await;

    // Get all the guilds the bot has access to.
    let guilds = ctx.cache.guilds().await.len();
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour((76, 187, 23))
                    .title(&user.name)
                    .thumbnail(&user.face())
                    .description(format!(
                        "I am a general purpose Discord bot, made with :heart: and Rust.\n\
                        I am currently running on {} servers.",
                        guilds,
                    ))
                    .field(
                        "Links",
                        "[Source](https://github.com/rsaihe/gemini-bot)",
                        false,
                    )
            })
        })
        .await?;

    Ok(())
}

#[command]
#[description("Get the invite link for the bot.")]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    // Get the current user.
    let user = ctx.cache.current_user().await;

    // Generate the invite URL.
    let permissions = Permissions::ADD_REACTIONS
        | Permissions::ATTACH_FILES
        | Permissions::EMBED_LINKS
        | Permissions::READ_MESSAGE_HISTORY
        | Permissions::READ_MESSAGES
        | Permissions::SEND_MESSAGES
        | Permissions::USE_EXTERNAL_EMOJIS;
    match user.invite_url(&ctx.http, permissions).await {
        Ok(url) => {
            msg.channel_id.say(&ctx, format!("<{}>", &url)).await?;
        }
        Err(e) => {
            msg.channel_id
                .say(&ctx, "There was a problem getting the invite link.")
                .await?;
            error!("Problem getting invite URL: {:?}", e);
        }
    };

    Ok(())
}

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
