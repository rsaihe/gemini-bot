use std::env;

use serenity::prelude::*;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file.
    dotenv::dotenv().expect("Failed to load .env file");

    // Retrieve token from environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Initialize client with token.
    let mut client = Client::builder(&token)
        .await
        .expect("Error creating client");

    // Start the client.
    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
