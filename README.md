<div align="center">
  <h1>🦀 crabbo</h1>
  <img src="https://img.shields.io/github/last-commit/ernstbecker/crabbo?&labelColor=151515&color=ff0043">
  <img src="https://img.shields.io/github/stars/ernstbecker/crabbo?style=flat&labelColor=151515&color=ff0043">
  <img src="https://img.shields.io/github/repo-size/ernstbecker/crabbo?&labelColor=151515&color=ff0043">
  <p>Discord bot built in Rust for interacting with the <a href="https://klever.org">Klever</a> blockchain.</p>
  <h3>🚧  Work in Progress  🚧</h3>
  <a href="#about">Commands</a>&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="#commands">Commands</a>&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="#requirements">Requirements</a>&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="#setup">Setup</a>&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="#docker">Docker</a>&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="#contributing">Contributing</a>
</div>

## About
Discord bot built in Rust for interacting with the <a href="https://klever.org">Klever</a> blockchain.

## Commands

| Command                       | Description       |
|-------------------------------|-------------------|
| `/ping`                       | Replies with pong |
| `/wallet <address> [network]` | Returns the KLV balance of a wallet address. Network defaults to `mainnet`, accepts `testnet` |

## Requirements

- [Rust](https://rustup.rs) 1.94+
- A Discord bot token ([Discord Developer Portal](https://discord.com/developers/applications))
- A Klever wallet address to test with (optional)

## Setup

**1. Clone the repository**

```sh
git clone https://github.com/ernstbecker/crabbo
cd crabbo
```

**2. Configure environment variables**

```sh
make setup
```

Edit `.env`:

```env
DISCORD_TOKEN=your_bot_token_here
GUILD_ID=your_guild_id_here   # optional — omit to register commands globally
```

> If `GUILD_ID` is set, commands are registered only for that server (instant update). Without it, commands are registered globally (may take up to 1 hour to propagate).

**3. Run**

```sh
make run
```

## Docker

```sh
docker compose up
```

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for project structure and instructions on adding new commands.

## License

GPL-3.0-or-later
