mod cache;
mod commands;
mod lib;

#[macro_use]
extern crate lazy_static;

use cache::{init_cache, CACHE_ENABLED};
use commands::{
    args::{parse, Commands},
    list::list_templates,
    pull::pull_template,
};

fn main() -> anyhow::Result<()> {
    if CACHE_ENABLED.to_owned() {
        init_cache()?;
    } else {
        println!("Cache is disabled");
        sleep_for!(1000);
    }

    let args = parse();

    if args.command == Commands::List {
        list_templates()?
    } else if args.command == Commands::Pull {
        pull_template()?
    }

    Ok(())
}
