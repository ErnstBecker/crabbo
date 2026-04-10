pub mod ping;

use std::collections::HashMap;
use async_trait::async_trait;
use serenity::builder::CreateCommand;
use serenity::model::application::CommandInteraction;
use serenity::prelude::Context;

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
    pub fn new() -> Self {
        let mut manager = CommandManager {
            commands: HashMap::new(),
        };
        manager.add(Box::new(ping::PingCommand));
        manager
    }

    fn add(&mut self, command: Box<dyn Command>) {
        self.commands.insert(command.name().to_string(), command);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Command> {
        self.commands.get(name).map(|c| c.as_ref())
    }

    pub fn all(&self) -> impl Iterator<Item = &dyn Command> {
        self.commands.values().map(|c| c.as_ref())
    }
}
