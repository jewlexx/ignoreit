use std::path::PathBuf;

use crate::cache;

#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    #[clap(
        short,
        long,
        help = "The path to save the template at",
        default_value = ".gitignore"
    )]
    output: PathBuf,

    #[clap(help = "Template name to pull")]
    template_name: Option<String>,
}

impl super::Command for Args {
    fn run(&self) -> anyhow::Result<()> {
        let cache = cache::Cache::open()?;

        println!("Loading template...");

        let template = if let Some(template_name) = &self.template_name {
            cache
                .find_template(template_name)
                .expect("template found in cache")
        } else {
            cache.pick_template()?
        };

        std::fs::copy(template.path(), &self.output)?;

        println!("Template saved to {}", self.output.display());

        Ok(())
    }
}
