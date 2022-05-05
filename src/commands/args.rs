use clap::{Arg, Parser, Subcommand};

use crate::{cache, lib::VERSION};

use super::{list::list_templates, pull::pull_template};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    #[clap(about = "List all available templates")]
    List,

    #[clap(about = "Pull a template from the repository", args = [
        Arg::new("template").takes_value(true).required(true),
        Arg::new("output").takes_value(true).required(false),
    ])]
    Pull,

    #[clap(about = "Purge gitignore cache")]
    Purge,
}

pub fn parse_args() -> anyhow::Result<()> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains("-V") || args.contains("-v") || args.contains("--version") {
        println!("{}", VERSION);
        return Ok(());
    }

    let sub = args.subcommand()?;
    let mut help = args.contains("--help") || args.contains("-h");

    if let Some(sub) = sub {
        match sub.as_ref() {
            "list" | "l" => list_templates()?,
            "pull" | "p" => pull_template()?,
            "purge" => cache::purge()?,
            "help" | "h" => help = true,
            _ => unreachable!(),
        }
    } else {
        help = true;
    }

    Ok(())
}
