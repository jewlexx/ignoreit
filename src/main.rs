mod cache;
mod commands;
mod lib;

#[macro_use]
extern crate lazy_static;

use cache::{init_cache, CACHE_ENABLED};
use colored::Colorize;
use commands::{
    args::{parse, Commands},
    list::list_templates,
    pull::pull_template,
    purge::purge,
};

fn main() -> anyhow::Result<()> {
    if CACHE_ENABLED.to_owned() {
        init_cache()?;
    } else {
        println!(
            "{}",
            "warning: cache is disabled. performance will not be optimal".yellow()
        );
        sleep_for!(3000);
    }

    let args = parse();

    if args.command == Commands::Purge {
        purge()?;
    } else if args.command == Commands::List {
        list_templates()?;
    } else if args.command == Commands::Pull {
        pull_template()?;
    }

    Ok(())
}
