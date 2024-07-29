#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    #[clap(from_global)]
    dry_run: bool,
}

impl super::Command for Args {
    fn run(&self) -> anyhow::Result<()> {
        println!("Purging cache...");

        Ok(())
    }
}
