//! CLI Arguments

use clap::{Parser, Subcommand};

use crate::{
    cache::{self, init_cache},
    commands::{list_templates, pull_template},
};

/// All possible CLI commands
#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Pull the template from the cache
    Pull {
        /// The name fo the template to pull
        template: Option<String>,

        /// The path to output the template to
        #[clap(short, long, default_value = ".gitignore")]
        output: String,

        /// Whether to append the template to the end an existing gitignore
        #[clap(long)]
        append: bool,

        /// Whether to overwrite the template if it already exists
        #[clap(long)]
        overwrite: bool,

        /// Whether to exit if the template already exists
        #[clap(long)]
        no_overwrite: bool,
    },

    /// List all available templates
    List,

    /// Purge the cache
    Purge,
}

impl Commands {
    /// Runs the subcommand
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Commands::List => {
                if *crate::cache::CACHE_ENABLED {
                    init_cache()?;
                }

                list_templates()?;
            }
            Commands::Pull {
                output,
                template,
                append,
                overwrite,
                no_overwrite,
            } => {
                if *crate::cache::CACHE_ENABLED {
                    init_cache()?;
                }

                pull_template(output, template.clone(), append, overwrite, no_overwrite)?;
            }
            Commands::Purge => cache::purge()?,
        };

        Ok(())
    }
}

/// CLI Args
#[derive(Parser, Clone, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// The command to execute
    #[clap(subcommand)]
    pub command: Commands,
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
    pub fn get_opt(append: &bool, overwrite: &bool, no_overwrite: &bool) -> Option<Self> {
        let mut opts_vec = Vec::<Self>::new();

        if *append {
            opts_vec.push(PullOpts::Append);
        }
        if *overwrite {
            opts_vec.push(PullOpts::Overwrite);
        }
        if *no_overwrite {
            opts_vec.push(PullOpts::NoOverwrite);
        }

        if opts_vec.len() > 1 {
            panic!("Only one pull option can be specified");
        }

        opts_vec.pop()
    }
}
