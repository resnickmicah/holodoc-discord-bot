# Running via Shuttle.rs #
(Assuming you already have your own Discord server where you want to install this bot and have cloned this repo to your local machine)
1. [Install Rust](https://forge.rust-lang.org/infra/other-installation-methods.html)
2. [Set up the bot on the Discord Developer Portal](https://discord.com/developers/docs/quick-start/getting-started#step-1-creating-an-app)
3. [Make a Shuttle account](https://console.shuttle.rs/login)
4. Add a Secrets.toml with your DISCORD_TOKEN for the app you set up in step 2.
5. `cargo check`, `cargo build`
6. `cargo install cargo-shuttle`, `cargo shuttle project start`, `cargo shuttle deploy`