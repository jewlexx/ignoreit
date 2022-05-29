use std::fmt::Display;

use clap::{Parser, Subcommand};
use const_strum::ConstStr;
use lazy_static::lazy_static;

use crate::cache;

use super::{list::list_templates, pull::pull_template};

#[derive(Debug, Subcommand, Clone, Copy, PartialEq, ConstStr)]
pub enum Commands {
    List,
    Pull,
    Purge,
    Help,
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.const_to_string())
    }
}

// TODO: Fix the help message and add help to subcommands
impl Commands {
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

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,

    #[clap(default_value = ".gitignore")]
    pub output: String,

    #[clap(short, long)]
    pub append: bool,

    #[clap(short, long)]
    pub overwrite: bool,

    #[clap(short, long)]
    pub no_overwrite: bool,
}

#[derive(PartialEq, Eq)]
pub enum PullOpts {
    Append,
    Overwrite,
    NoOverwrite,
}

impl PullOpts {
    pub fn get_opt(args: &Args) -> Option<Self> {
        let mut opts_vec = Vec::<Self>::new();

        if args.append {
            opts_vec.push(PullOpts::Append);
        }
        if args.overwrite {
            opts_vec.push(PullOpts::Overwrite);
        }
        if args.no_overwrite {
            opts_vec.push(PullOpts::NoOverwrite);
        }

        if opts_vec.len() > 1 {
            panic!("Only one pull option can be specified");
        }

        opts_vec.pop()
    }
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}
