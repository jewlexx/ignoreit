mod commands;
mod lib;
mod remote;
mod sys;

#[macro_use]
mod macros;

mod cache;

use clap::StructOpt;
use commands::{args::Commands, list::list_templates, pull::pull_template};

use crate::commands::args::Args;

fn main() -> anyhow::Result<()> {
    if lib::CACHE_ENABLED.to_owned() {
        cache::init_cache()?;
    } else {
        use colored::Colorize;
        if !lib::IS_ONLINE.to_owned() {
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

    match args.command {
        Commands::List => list_templates()?,
        Commands::Pull => pull_template()?,
        Commands::Purge => cache::purge()?,
    }

    Ok(())
}
