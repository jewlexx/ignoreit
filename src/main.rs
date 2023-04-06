//! CLI application to pull gitignore templates with ease

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use clap::Parser;

pub mod cache;
pub mod commands;
pub mod macros;
pub mod templates;

// TODO: add custom errors with `thiserror`

use commands::args::Args;

fn main() -> anyhow::Result<()> {
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
