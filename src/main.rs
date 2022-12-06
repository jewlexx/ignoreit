//! CLI application to pull gitignore templates with ease

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::time::SystemTime;

use clap::Parser;

use lazy_static::lazy_static;

// TODO: Refactor cache into struct

lazy_static! {
    /// The current time in milliseconds
    pub static ref TIMESTAMP: u128 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_millis();
}

pub mod cache;
pub mod commands;
pub mod macros;

// TODO: add custom errors with `thiserror`

use commands::args::Args;

fn main() -> anyhow::Result<()> {
    lazy_static::initialize(&TIMESTAMP);

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
