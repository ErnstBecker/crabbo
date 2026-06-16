# Contributing to crabbo

## Commands

| Command                       | Description       |
|-------------------------------|-------------------|
| `/ping`                       | Replies with pong |
| `/wallet <address> [network]` | Returns the KLV balance of a wallet address. Network defaults to `mainnet`, accepts `testnet` |

## Makefile

| Command      | Description                                                      |
|--------------|------------------------------------------------------------------|
| `make setup` | Copies `.env.example` to `.env` (skips if already exists) |
| `make run`   | Runs the bot in development mode                          |
| `make build` | Compiles the release binary to `target/release/crabbo`    |
| `make fmt`   | Applies `rustfmt` formatting to all files                 |
| `make check` | Verifies formatting and runs `clippy`                     |

## Project structure

```
src/
├── main.rs               # Bootstrap: reads config, starts the Discord client
├── commands/
│   ├── mod.rs            # Command trait, CommandManager, and registration
│   ├── ping.rs
│   └── wallet.rs
├── handlers/
│   └── mod.rs            # Discord event handler (ready, interaction_create)
├── client/
│   └── klever.rs         # HTTP client for the Klever node API
├── services/
│   └── wallet.rs         # Wallet balance business logic
└── utils/                # Shared utilities
```

### How a command works

Every command implements the `Command` trait defined in `src/commands/mod.rs`:

- `name()` — the slash command name registered on Discord
- `register()` — builds the command definition sent to the Discord API (description, options, etc.)
- `run()` — executes when a user invokes the command

The `CommandManager` stores all commands in a `HashMap<name → command>`. On bot startup, `handlers/mod.rs` reads all registered commands via `command_manager.all()` and registers them with Discord. On each interaction, it calls `command_manager.get(name)` and dispatches to `run()`.

### Flow of an interaction

```
Discord → interaction_create → CommandManager::get(name) → cmd.run()
```

## Adding a new command

**1.** Create `src/commands/your_command.rs` and implement the `Command` trait:

```rust
use async_trait::async_trait;
use serenity::builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::CommandInteraction;
use serenity::prelude::Context;
use super::Command;

pub struct YourCommand;

#[async_trait]
impl Command for YourCommand {
    fn name(&self) -> &str {
        "your-command"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new("your-command").description("Does something")
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("response here"),
        );
        if let Err(why) = interaction.create_response(&ctx.http, response).await {
            eprintln!("Failed to respond to /your-command: {:?}", why);
        }
    }
}
```

**2.** Register it in `src/commands/mod.rs`:

```rust
pub mod your_command;
use your_command::YourCommand;

// inside CommandManager::build():
.register(YourCommand)
```

If your command needs a service, add it to the `Services` struct and pass it through:

```rust
struct Services {
    wallet: Arc<WalletService>,
    your_service: Arc<YourService>,
}

impl Services {
    fn new() -> Self {
        Self {
            wallet: Arc::new(WalletService::new(Arc::new(KleverClient::default()))),
            your_service: Arc::new(YourService::new()),
        }
    }
}

// inside CommandManager::build():
use your_command::YourCommand;

.register(YourCommand { service: services.your_service })
```
