use anyhow::Context;
use clap::Subcommand;

use crate::cache::CacheHandler;

mod list;
mod pull;
mod purge;

/// All possible CLI commands
#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Pull the template from the cache
    Pull(pull::Args),

    /// List all available templates
    List(list::Args),

    /// Purge the cache
    Purge,
}

impl Command for Commands {
    fn run(&self, cache: CacheHandler) -> anyhow::Result<()> {
        cache.init_cache().context("Failed to initialize cache")?;

        match self {
            Commands::List(args) => args.run(cache)?,
            Commands::Pull(args) => args.run(cache)?,
            Commands::Purge => {
                cache.purge()?;
            }
        };

        Ok(())
    }
}

pub trait Command {
    /// Runs the subcommand
    fn run(&self, cache: CacheHandler) -> anyhow::Result<()>;
}

/// The list of options the user can give when the gitignore exists
#[derive(PartialEq, Eq)]
pub enum PullOpts {
    /// Append to the current gitignore
    Append,
    /// Overwrite the current gitignore
    Overwrite,
    /// Exit if there is an existing gitignore
    NoOverwrite,
}

impl PullOpts {
    /// Get the option based on a boolean representation
    pub fn get_opt(append: bool, overwrite: bool, no_overwrite: bool) -> Option<Self> {
        let mut opts_vec = Vec::<Self>::new();

        if append {
            opts_vec.push(PullOpts::Append);
        }
        if overwrite {
            opts_vec.push(PullOpts::Overwrite);
        }
        if no_overwrite {
            opts_vec.push(PullOpts::NoOverwrite);
        }

        if opts_vec.len() > 1 {
            panic!("Only one pull option can be specified");
        }

        opts_vec.pop()
    }
}
