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
    let args = parse();

    if CACHE_ENABLED.to_owned() {
        init_cache()?;
    }

    if args.command == Commands::List {
        list_templates()?
    } else if args.command == Commands::Pull {
        pull_template()?
    }

    Ok(())
}
