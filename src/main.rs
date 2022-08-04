//! CLI application to pull gitignore templates with ease

#![forbid(unsafe_code)]
#![warn(missing_docs)]
use clap::Parser;

pub mod cache;
pub mod commands;
pub mod macros;
pub mod templates;
pub mod utils;

// TODO: add custom errors with `thiserror`

use commands::args::Args;

fn main() -> anyhow::Result<()> {
    lazy_static::initialize(&utils::TIMESTAMP);

    if *utils::CACHE_ENABLED {
        cache::init_cache()?;
    } else {
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
