use std::fmt::Display;

use colored::Colorize;
use lazy_static::lazy_static;
use pico_args::Arguments;

use crate::{
    cache,
    utils::{DESC, VERSION},
};

use super::{list::list_templates, pull::pull_template};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Commands {
    List,
    Pull,
    Purge,
    Help,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PullOpts {
    Append,
    Overwrite,
    NoOverwrite,
}

impl Display for PullOpts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PullOpts::Append => write!(f, "{}ppend", "A".underline()),
            PullOpts::Overwrite => write!(f, "{}verwrite", "O".underline()),
            PullOpts::NoOverwrite => write!(f, "{}o-verwrite", "N".underline()),
        }
    }
}

impl PullOpts {
    pub fn get_opts<'a>(&self) -> [&'a str; 2] {
        match *self {
            PullOpts::Append => ["-A", "--append"],
            PullOpts::Overwrite => ["-O", "--overwrite"],
            PullOpts::NoOverwrite => ["-N", "--no-overwrite"],
        }
    }

    pub fn get_args(args: &mut Arguments) -> Vec<Self> {
        let opts = [PullOpts::Append, PullOpts::Overwrite, PullOpts::NoOverwrite];

        opts.iter()
            .filter(|x| {
                let opts = x.get_opts();

                args.contains(opts)
            })
            .copied()
            .collect()
    }
}

// TODO: Fix the help message and add help to subcommands

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
            Commands::Pull => String::from("pull"),
            _ => self.to_string(),
        }
    }

    fn get_info(self) -> String {
        format!("{0:<10} {1}", self.get_usage().green(), self.get_help())
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

#[derive(Default, Clone, Debug)]
pub struct Args {
    pub command: Option<Commands>,
    pub output: Option<String>,
    pub pull_opt: Option<PullOpts>,
}

impl Args {
    pub fn parse() -> Self {
        let mut args = pico_args::Arguments::from_env();

        if args.contains(["-V", "--version"]) {
            println!("{}", VERSION);
            return Default::default();
        }

        let sub = args.subcommand().unwrap();
        let command = Commands::from_str(&sub.unwrap_or_else(|| String::from("help")));
        let help = args.contains(["-h", "--help"])
            || match command {
                Some(v) => v == Commands::Help,
                None => true,
            };

        if help {
            println!("{} {}", "ignoreit".green(), VERSION.yellow());
            println!();
            println!("{}", DESC);
            println!();
            println!(
                "{}:\n   ignoreit [FLAGS] <COMMAND> [ARGUMENTS]",
                "USAGE".yellow()
            );
            println!();
            println!("{}:", "FLAGS".yellow());
            println!(
                "   {}     {}",
                "-h, --help".green(),
                Commands::Help.get_help()
            );
            println!("   {}  Show version", "-V, --version".green());
            println!();
            println!("{}:", "COMMANDS".yellow());
            println!("   {}", Commands::Help.get_info());
            println!("   {}", Commands::List.get_info());
            println!("   {}", Commands::Pull.get_info(),);
            println!("   {}", Commands::Purge.get_info());
            println!();
            println!(
                "Thank you for using {} by Juliette Cordor",
                "ignoreit".green()
            );
        }

        let output = args
            .opt_value_from_str::<[&str; 2], String>(["-o", "--output"])
            .unwrap();

        let pull_opt = {
            let opts = PullOpts::get_args(&mut args);

            if opts.len() > 1 {
                println!("{}", "Only one pull option can be used at a time".red());
                None
            } else {
                opts.first().copied()
            }
        };

        Args {
            command,
            output,
            pull_opt,
        }
    }
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}
