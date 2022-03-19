use anyhow::Result;
use clap::Parser;

mod args;
mod commands;
mod lib;

use args::{Args, Commands};
use commands::{list::list_templates, pull::pull_template};

fn main() -> Result<()> {
    let args = Args::parse();

    if args.command == Commands::List {
        list_templates()?
    } else if args.command == Commands::Pull {
        pull_template()?
    }

    Ok(())
}
