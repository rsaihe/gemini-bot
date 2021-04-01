use rand::seq::SliceRandom;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

static EIGHT_BALL_RESPONSES: [&str; 20] = [
    "It is certain.",
    "It is decidedly so.",
    "Without a doubt.",
    "Yes\u{2014}definitely.",
    "You may rely on it.",
    "Most likely.",
    "As I see it, yes.",
    "Outlook good.",
    "Yes.",
    "Signs point to yes.",
    "Reply hazy, try again.",
    "Ask again later.",
    "Better not tell you now.",
    "Cannot predict now.",
    "Concentrate and ask again.",
    "Don't count on it.",
    "My reply is no.",
    "My sources say no.",
    "Outlook not so good.",
    "Very doubtful.",
];

#[group]
#[commands(eight_ball, shuffle)]
struct Fun;

#[command("8ball")]
#[description("Ask the Magic 8-Ball a question.")]
async fn eight_ball(ctx: &Context, msg: &Message) -> CommandResult {
    // Generate a random response.
    let response = {
        let mut rng = rand::thread_rng();
        EIGHT_BALL_RESPONSES.choose(&mut rng).unwrap()
    };
    msg.channel_id.say(&ctx, response).await?;

    Ok(())
}

#[command]
#[description("Randomly shuffle words.")]
#[min_args(1)]
#[usage("<word>...")]
async fn shuffle(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Shuffle graphemes in each Unicode word.
    let words: String = args
        .message()
        .trim()
        .split_word_bounds()
        .map(|w| {
            let mut graphemes: Vec<_> = w.graphemes(true).collect();
            graphemes.shuffle(&mut rand::thread_rng());
            graphemes.concat()
        })
        .collect();

    msg.channel_id.say(&ctx, &words).await?;

    Ok(())
}
