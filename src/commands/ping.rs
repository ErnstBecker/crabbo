use async_trait::async_trait;
use serenity::builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::CommandInteraction;
use serenity::prelude::Context;

use super::Command;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    fn name(&self) -> &str {
        "ping"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new("ping").description("Replies with pong!")
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("pong!"),
        );

        if let Err(why) = interaction.create_response(&ctx.http, response).await {
            eprintln!("Failed to respond to /ping: {:?}", why);
        }
    }
}
