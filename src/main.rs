#![forbid(unsafe_code)]
use clap::Parser;

mod cache;
mod commands;
mod macros;
mod utils;

use commands::args::Args;

fn main() -> anyhow::Result<()> {
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

    if let Some(cmd) = args.command {
        cmd.run()?;
    }

    Ok(())
}
