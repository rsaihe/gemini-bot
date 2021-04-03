# gemini-bot

A general-purpose Discord bot.

## Setup

Create a new bot application on Discord's [developer portal]. Next, copy the
`.env.example` file and rename it to `.env`. Add the application token to the
end of the `DISCORD_TOKEN=` line.

Use the following link in order to invite the bot to a server, replacing
`CLIENT_ID` with the client ID found on the developer portal:

<https://discord.com/api/oauth2/authorize?client_id=CLIENT_ID&scope=bot&permissions=379968>

The bot can then be run with `cargo run --release`.

[developer portal]: https://discord.com/developers/applications
