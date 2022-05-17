use std::fmt::Display;

use lazy_static::lazy_static;

use crate::{
    cache,
    lib::{DESC, VERSION},
};

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

    fn get_info(self) -> String {
        format!("   {0: <25} {1}", self.get_usage(), self.get_help())
    }

    pub fn run(self) -> anyhow::Result<()> {
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

pub fn parse_args() -> anyhow::Result<Option<Commands>> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains("-V") || args.contains("--version") {
        println!("{}", VERSION);
        return Ok(None);
    }

    let sub = args.subcommand()?;
    let command = Commands::from_str(&sub.unwrap_or_else(|| String::from("help")));
    let help = args.contains("--help")
        || args.contains("-h")
        || match command {
            Some(v) => v == Commands::Help,
            None => true,
        };

    use colored::Colorize;
    if help {
        println!("{} {}", "ignoreit".green(), VERSION.yellow());
        println!();
        println!("{}", DESC);
        println!();
        println!(
            "{}: ignoreit [FLAGS] <COMMAND> [ARGUMENTS]",
            "Usage".green()
        );
        println!();
        println!("{}:", "Flags".green());
        println!("  -h, --help     {}", Commands::Help.get_help());
        println!("  -V, --version  Show version");
        println!();
        println!("{}:", "Commands".green());
        println!("{}", Commands::Help.get_info());
        println!("{}", Commands::List.get_info());
        println!("{}", Commands::Pull.get_info(),);
        println!("{}", Commands::Purge.get_info());
        println!();
        println!(
            "Thank you for using {} by Juliette Cordor",
            "ignoreit".green()
        );
    }

    Ok(command)
}
