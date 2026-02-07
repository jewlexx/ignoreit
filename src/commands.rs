use clap::Subcommand;

use crate::cache::CacheHandler;

mod list;
mod pull;
mod search;

/// All possible CLI commands
#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Pull the template from the cache
    Pull(pull::Args),

    /// List all available templates
    List(list::Args),

    /// Search all available templates
    Search(search::Args),

    /// Purge the cache
    Purge,
}

impl Command for Commands {
    async fn run(&self, cache: CacheHandler) -> anyhow::Result<()> {
        match self {
            Commands::List(args) => args.run(cache).await?,
            Commands::Pull(args) => args.run(cache).await?,
            Commands::Search(args) => args.run(cache).await?,
            Commands::Purge => cache.purge()?,
        };

        Ok(())
    }
}

pub trait Command {
    /// Runs the subcommand
    fn run(
        &self,
        cache: CacheHandler,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
