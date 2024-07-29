use crate::{CACHE, CONFIG};

#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    #[clap(from_global)]
    dry_run: bool,
}

impl super::Command for Args {
    const INTERRUPT_BACKGROUND_TASK: bool = true;

    async fn run(&self) -> anyhow::Result<()> {
        CACHE.purge()?;
        let mut config = CONFIG.lock().await;

        config.first_run = true;
        config.save()?;

        println!("Cache purged");
        println!("Please run ignoreit again to reload the cache");

        Ok(())
    }
}
