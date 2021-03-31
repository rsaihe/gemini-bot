use std::collections::HashSet;
use std::time::Instant;

use serenity::framework::standard::macros::{command, group, help};
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

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
