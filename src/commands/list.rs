use crate::CACHE;

#[derive(Debug, Clone, clap::Parser)]
pub struct Args;

impl super::Command for Args {
    async fn run(&self) -> anyhow::Result<()> {
        println!("Available Templates:");

        for template in CACHE.list_templates() {
            println!("\t{}", template.name());
        }

        Ok(())
    }
}
