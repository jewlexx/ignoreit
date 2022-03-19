use anyhow::Result;

mod args;
mod commands;
mod lib;

use args::{parse, Commands};
use commands::{list::list_templates, pull::pull_template};

fn main() -> Result<()> {
    let args = parse();

    if args.command == Commands::List {
        list_templates()?
    } else if args.command == Commands::Pull {
        pull_template()?
    }

    Ok(())
}
