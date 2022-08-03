#![forbid(unsafe_code)]
use std::time::SystemTime;

use clap::Parser;

mod cache;
mod commands;
mod macros;
mod templates;
mod utils;

use commands::args::Args;

lazy_static::lazy_static! {
    static ref TIMESTAMP: u128 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("time went backwards").as_millis();
}

fn main() -> anyhow::Result<()> {
    lazy_static::initialize(&TIMESTAMP);

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
