use crate::{cache, lib::VERSION};

use super::{list::list_templates, pull::pull_template};

pub struct Args {
    pub command: Commands,
}

pub enum Commands {
    /// List all available templates
    List,

    /// Pull a template from the repository <template> <output?>
    Pull,

    /// Purge gitignore cache
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
