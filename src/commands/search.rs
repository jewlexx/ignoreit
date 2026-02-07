use clap::Parser;

use crate::cache::CacheHandler;

#[derive(Debug, Parser, Clone)]
pub struct Args {
    /// Query to search for
    #[clap()]
    query: String,
}

impl super::Command for Args {
    async fn run(&self, cache: CacheHandler) -> anyhow::Result<()> {
        let templates = cache.search_templates(&self.query).await?;

        println!("Available templates:");

        for item in templates {
            println!("  {}", item);
        }

        println!("\nEnter one of the above names eg. Rust");

        Ok(())
    }
}
