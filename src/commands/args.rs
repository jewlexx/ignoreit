use crate::{cache, lib::VERSION};

use super::{list::list_templates, pull::pull_template};

pub enum Commands {
    /// List all available templates
    List,

    /// Pull a template from the repository <template> <output?>
    Pull,

    /// Purge gitignore cache
    Purge,

    /// Help Command
    Help,
}

impl Commands {
    fn from_str(command: &str) -> Option<Self> {
        match command {
            "list" | "l" => Some(Commands::List),
            "pull" | "p" => Some(Commands::Pull),
            "purge" => Some(Commands::Purge),
            "help" | "h" => Some(Commands::Help),
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Commands::List => String::from("list"),
            Commands::Pull => String::from("pull"),
            Commands::Purge => String::from("purge"),
            Commands::Help => String::from("help"),
        }
    }

    fn get_help(self) -> String {
        match self {
            Commands::List => String::from("List all available templates"),
            Commands::Pull => String::from("Pull a template from the repository"),
            Commands::Purge => String::from("Purge gitignore cache"),
            Commands::Help => String::from("Shows the help message"),
        }
    }

    fn get_usage(self) -> String {
        match self {
            Commands::Pull => String::from("pull <template> [output]"),
            _ => self.to_string(),
        }
    }
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
        let command = Commands::from_str(&sub);

        if let Some(command) = command {
            match command {
                Commands::List => list_templates()?,
                Commands::Pull => pull_template()?,
                Commands::Purge => cache::purge()?,
                Commands::Help => help = true,
            }
        } else {
            help = true;
        }
    } else {
        help = true;
    }

    if help {
        println!("{}", include_str!("help.txt"));
    }

    Ok(())
}
