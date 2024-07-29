use std::path::PathBuf;

#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    #[clap(default_value = ".gitignore")]
    output: PathBuf,
}

impl super::Command for Args {
    fn run(&self) -> anyhow::Result<()> {
        println!("Pulling template...");

        Ok(())
    }
}
