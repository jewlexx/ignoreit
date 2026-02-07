use std::{env, fs::OpenOptions, io::Write};

use anyhow::Context;
use clap::Parser;

use crate::{cache::CacheHandler, commands::PullOpts};

#[derive(Debug, Parser, Clone)]
pub struct Args {
    /// The name of the template to pull
    template: String,

    /// The path to output the template to
    #[clap(short, long, default_value = ".gitignore")]
    output: String,

    /// Whether to append the template to the end an existing gitignore
    #[clap(long)]
    append: bool,

    /// Whether to overwrite the template if it already exists
    #[clap(long)]
    overwrite: bool,

    /// Whether to exit if the template already exists
    #[clap(long)]
    no_overwrite: bool,
}

impl super::Command for Args {
    async fn run(&self, cache: CacheHandler) -> anyhow::Result<()> {
        let template = cache.get_template(&self.template).await?;

        let path = env::current_dir()
            .with_context(|| "Failed to get current directory")?
            .join(&self.output);

        let mut openopts = OpenOptions::new();
        openopts.create(true);
        openopts.write(true);

        if path.exists() {
            let pull_opt = PullOpts::get_opt(self.append, self.overwrite, self.no_overwrite);
            let opt = pull_opt
                .map(anyhow::Ok)
                .unwrap_or_else(|| -> anyhow::Result<PullOpts> {
                    use dialoguer::{theme::ColorfulTheme, Select};

                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("The gitignore file already exists in your current directory")
                        .items(["Append", "Overwrite", "Exit"])
                        .default(0)
                        .interact()?;

                    Ok(match selection {
                        0 => PullOpts::Append,
                        1 => PullOpts::Overwrite,
                        // 2 and anything else
                        _ => PullOpts::NoOverwrite,
                    })
                })?;

            match opt {
                PullOpts::NoOverwrite => {
                    println!("Goodbye!");
                    return Ok(());
                }
                PullOpts::Append => {
                    // Append written content to the end of the existing file
                    openopts.append(true);
                }
                PullOpts::Overwrite => {
                    openopts.write(true);
                    openopts.truncate(true);
                }
            }
        }

        let mut file = openopts.open(&path)?;

        println!("Getting template {}", template.name);
        writeln!(file, "# {}", template.file_name)?;
        write!(file, "{}", template.contents)?;

        Ok(())
    }
}
