use serenity::async_trait;
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::commands::CommandManager;

pub struct EventHandler {
    pub command_manager: CommandManager,
}

#[async_trait]
impl serenity::client::EventHandler for EventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} connected!", ready.user.name);

        let commands = self.command_manager.all().map(|c| c.register()).collect::<Vec<_>>();

        if let Err(why) = Command::set_global_commands(&ctx.http, commands).await {
            eprintln!("Failed to register commands: {:?}", why);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction
            && let Some(cmd) = self.command_manager.get(&command.data.name)
        {
            cmd.run(&ctx, &command).await;
        }
    }
}
