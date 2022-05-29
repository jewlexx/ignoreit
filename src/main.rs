#![forbid(unsafe_code)]
use clap::Parser;

mod cache;
mod commands;
mod macros;
mod net;
mod utils;

use commands::args::Args;

fn main() -> anyhow::Result<()> {
    if *utils::CACHE_ENABLED {
        cache::init_cache()?;
    } else {
        use colored::Colorize;
        if !(*utils::IS_ONLINE) {
            println!(
                "{}",
                "error: you are offline and cache is disabled. we cannot continue".red()
            );
            std::process::exit(1);
        }

        println!(
            "{}",
            "warning: cache is disabled. performance will not be optimal".yellow()
        );
        sleep_for!(3000);
    }

    let args = Args::parse();

    if let Some(cmd) = args.command {
        cmd.run()?;
    }

    Ok(())
}
