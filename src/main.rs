//! CLI application to pull gitignore templates with ease

use std::{
    sync::LazyLock,
    time::{Duration, SystemTime},
};

use clap::Parser;

pub static STARTUP_TIMESTAMP: LazyLock<Duration> = LazyLock::new(|| {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
});

pub mod cache;
pub mod commands;
pub mod macros;
pub mod templates;

use commands::args::Args;

fn main() -> anyhow::Result<()> {
    LazyLock::force(&STARTUP_TIMESTAMP);

    if !*cache::CACHE_ENABLED {
        use mincolor::Colorize;
        println!(
            "{}",
            "warning: cache is disabled. performance will not be optimal".yellow()
        );
        sleep_for!(3000);
    }

    let args = Args::parse();

    args.command.run()?;

    Ok(())
}
