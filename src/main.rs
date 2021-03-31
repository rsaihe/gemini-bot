use std::collections::HashSet;
use std::env;

use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use commands::*;

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file.
    dotenv::dotenv().expect("Failed to load .env file");

    // Initialize the logger with environemnt variables.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start logger");

    // Retrieve token from environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Store bot owner.
    let http = Http::new_with_token(&token);
    let owners = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            owners
        }
        Err(e) => panic!("Could not access application info: {:?}", e),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("?"))
        .group(&META_GROUP);

    // Initialize client with token.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Start the client.
    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }
}
