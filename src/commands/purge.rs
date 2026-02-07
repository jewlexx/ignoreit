use crate::cache::CacheHandler;

pub struct Args;

impl super::Command for Args {
    async fn run(&self, cache: CacheHandler) -> anyhow::Result<()> {
        cache.purge()?;
        println!("Cache purged successfully");
        Ok(())
    }
}
