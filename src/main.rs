mod commands;
mod lib;

#[macro_use]
extern crate macros;

use commands::{
    args::{parse, Commands},
    list::list_templates,
    pull::pull_template,
};

fn main() -> anyhow::Result<()> {
    if cfg!(feature = "cache") {
        use colored::Colorize;
        if cache::CACHE_ENABLED.to_owned() {
            cache::init_cache()?;
        } else {
            println!(
                "{}",
                "warning: cache is disabled. performance will not be optimal".yellow()
            );
            sleep_for!(3000);
        }
    }

    let args = parse();

    if args.command == Commands::List {
        return list_templates();
    }
    if args.command == Commands::Pull {
        return pull_template();
    }
    #[cfg(feature = "cache")]
    if args.command == Commands::Purge {
        return cache::purge();
    }

    Ok(())
}
