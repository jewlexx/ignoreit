use std::fmt::Display;

use crate::{cache, lib::VERSION};

use super::{list::list_templates, pull::pull_template};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Commands {
    List,
    Pull,
    Purge,
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

    fn run(self) -> anyhow::Result<()> {
        match self {
            Commands::List => list_templates()?,
            Commands::Pull => pull_template()?,
            Commands::Purge => cache::purge()?,
            _ => (),
        };

        Ok(())
    }
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Commands::List => String::from("list"),
            Commands::Pull => String::from("pull"),
            Commands::Purge => String::from("purge"),
            Commands::Help => String::from("help"),
        };

        write!(f, "{}", s)
    }
}

pub fn parse_args() -> anyhow::Result<()> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains("-V") || args.contains("-v") || args.contains("--version") {
        println!("{}", VERSION);
        return Ok(());
    }

    let sub = args.subcommand()?;
    let command = Commands::from_str(&sub.unwrap_or_else(|| String::from("help")));
    let help = args.contains("--help")
        || args.contains("-h")
        || match command {
            Some(v) => v == Commands::Help,
            None => true,
        };

    if help {
        println!("ignoreit {}", VERSION);
        println!("");
        println!("Usage:");
        println!(
            "  {0:<25} {1}",
            Commands::Help.get_usage(),
            Commands::Help.get_help()
        );
        println!(
            "  {0:<25} {1}",
            Commands::List.get_usage(),
            Commands::List.get_help()
        );
        println!(
            "  {0:<25} {1}",
            Commands::Pull.get_usage(),
            Commands::Pull.get_help()
        );
        println!(
            "  {0:<25} {1}",
            Commands::Purge.get_usage(),
            Commands::Purge.get_help()
        );
    }

    if let Some(command) = command {
        command.run()?;
    }

    Ok(())
}
