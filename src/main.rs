mod client;
mod commands;
mod handlers;
mod models;
mod services;
mod utils;

use serenity::prelude::*;
use handlers::EventHandler;
use commands::CommandManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN")
        .expect("Token not found");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let commands = CommandManager::build();

    let mut client = Client::builder(&token, intents)
        .event_handler(EventHandler {
            command_manager: commands,
        })
        .await?;

    if let Err(why) = client.start().await {
        eprintln!("Error starting bot: {:?}", why);
    }

    Ok(())
}
