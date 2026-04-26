mod client;
mod commands;
mod handlers;
mod models;
mod services;
mod utils;

use std::sync::Arc;
use serenity::prelude::*;
use handlers::EventHandler;
use commands::CommandManager;
use commands::ping::PingCommand;
use commands::wallet::WalletCommand;
use client::klever::KleverClient;
use services::wallet::WalletService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN")
        .expect("Token not found");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let wallet_service = Arc::new(WalletService::new(Arc::new(KleverClient::default())));

    let commands = vec![
        Box::new(PingCommand) as Box<dyn commands::Command>,
        Box::new(WalletCommand { service: wallet_service }) as Box<dyn commands::Command>,
    ];

    let mut client = Client::builder(&token, intents)
        .event_handler(EventHandler {
            command_manager: CommandManager::new(commands),
        })
        .await?;

    if let Err(why) = client.start().await {
        eprintln!("Error starting bot: {:?}", why);
    }

    Ok(())
}
