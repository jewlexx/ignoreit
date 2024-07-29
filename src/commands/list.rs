#[derive(Debug, Clone, clap::Parser)]
pub struct Args;

impl super::Command for Args {
    fn run(&self) -> anyhow::Result<()> {
        println!("Listing templates...");

        Ok(())
    }
}
