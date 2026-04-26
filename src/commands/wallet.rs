use std::sync::Arc;

use async_trait::async_trait;
use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{CommandInteraction, CommandOptionType};
use serenity::prelude::Context;

use super::Command;
use crate::services::wallet::WalletService;

pub struct WalletCommand {
    pub service: Arc<WalletService>,
}

#[async_trait]
impl Command for WalletCommand {
    fn name(&self) -> &str {
        "wallet"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new("wallet")
            .description("Replies with the wallet balance")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "address", "Wallet address")
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "network", "ex: 'mainnet', 'testnet'")
                .required(false),
            )
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) {
        let address = interaction
            .data
            .options
            .iter()
            .find(|o| o.name == "address")
            .and_then(|o| o.value.as_str())
            .unwrap_or("unknown");
        let short_address = crate::utils::abbrev_address(address);

        let network = interaction
            .data
            .options
            .iter()
            .find(|o| o.name == "network")
            .and_then(|o| o.value.as_str())
            .unwrap_or("mainnet");

        let content = match self.service.get_balance(address, network).await {
            Ok(balance) => format!("Network: `{}`\nWallet: `{}`\nBalance: `{}`", network, short_address, balance),
            Err(e) => format!("Error: {}", e),
        };

        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(content),
        );

        if let Err(why) = interaction.create_response(&ctx.http, response).await {
            eprintln!("Failed to respond to /wallet: {:?}", why);
        }
    }
}
