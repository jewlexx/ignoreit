use clap::Parser;

use crate::cache::CacheHandler;

#[derive(Debug, Parser, Copy, Clone)]
pub struct Args;

impl super::Command for Args {
    async fn run(&self, cache: CacheHandler) -> anyhow::Result<()> {
        let templates = cache.get_template_paths()?;

        println!("Available templates:");

        for item in templates {
            println!("  {}", item);
        }

        println!("\nEnter one of the above names eg. Rust");

        Ok(())
    }
}
