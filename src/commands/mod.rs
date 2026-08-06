pub mod ping;
pub mod wallet;

use ping::PingCommand;
use wallet::WalletCommand;

use crate::services::wallet::WalletService;
use async_trait::async_trait;
use serenity::builder::CreateCommand;
use serenity::model::application::CommandInteraction;
use serenity::prelude::Context;
use std::collections::HashMap;

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &str;
    fn register(&self) -> CreateCommand;
    async fn run(&self, ctx: &Context, interaction: &CommandInteraction);
}

pub struct CommandManager {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandManager {
    pub fn build(wallet: WalletService) -> Self {
        Self::new()
            .register(PingCommand)
            .register(WalletCommand { service: wallet })
    }

    fn new() -> Self {
        CommandManager {
            commands: HashMap::new(),
        }
    }

    pub fn register(mut self, command: impl Command + 'static) -> Self {
        let command = Box::new(command);
        self.commands.insert(command.name().to_string(), command);
        self
    }

    pub fn get(&self, name: &str) -> Option<&dyn Command> {
        self.commands.get(name).map(|c| c.as_ref())
    }

    pub fn all(&self) -> impl Iterator<Item = &dyn Command> {
        self.commands.values().map(|c| c.as_ref())
    }
}
