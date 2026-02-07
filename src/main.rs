//! CLI application to pull gitignore templates with ease

use std::{
    sync::LazyLock,
    time::{Duration, SystemTime},
};

use clap::Parser;
use commands::Command;

pub static STARTUP_TIMESTAMP: LazyLock<Duration> = LazyLock::new(|| {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
});

mod api;
pub mod cache;
pub mod commands;
pub mod macros;
pub mod templates;

/// CLI Args
#[derive(Parser, Clone, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// The command to execute
    #[clap(subcommand)]
    pub command: commands::Commands,

    /// Force update cache
    #[clap(short = 'F', long, default_value = "false")]
    pub force_update: bool,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    LazyLock::force(&STARTUP_TIMESTAMP);

    let cache = cache::CacheHandler::new().await?;

    let args = Args::parse();

    cache.update(args.force_update).await?;
    args.command.run(cache).await?;

    Ok(())
}
