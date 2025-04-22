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
}

fn main() -> anyhow::Result<()> {
    LazyLock::force(&STARTUP_TIMESTAMP);

    let Some(cache) = cache::CacheHandler::new() else {
        anyhow::bail!("failed to initialize cache");
    };

    let args = Args::parse();

    args.command.run(cache)?;

    Ok(())
}
