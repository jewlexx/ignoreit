#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    #[clap(from_global)]
    dry_run: bool,
}

impl super::Command for Args {
    const INTERRUPT_BACKGROUND_TASK: bool = true;

    async fn run(&self) -> anyhow::Result<()> {
        unimplemented!();

        println!("Cache purged");
        println!("Please run ignoreit again to reload the cache");

        Ok(())
    }
}
