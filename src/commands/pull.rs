use std::{env, fs::OpenOptions, io::Write};

use anyhow::Context;
use clap::Parser;

use crate::cache::CacheHandler;

#[derive(Debug, Copy, Clone, Parser)]
#[group(required = true, multiple = false)]
pub struct OverwriteOpts {
    /// Append the template to the end an existing gitignore
    #[clap(long)]
    append: bool,
    /// Overwrite the template if it already exists
    #[clap(long)]
    overwrite: bool,
    /// Exit if the template already exists
    #[clap(long)]
    no_overwrite: bool,
}

#[derive(Debug, Parser, Clone)]
pub struct Args {
    /// The name of the template to pull
    template: String,

    /// The path to output the template to
    #[clap(short, long, default_value = ".gitignore")]
    output: String,

    #[clap(flatten)]
    overwite_opts: Option<OverwriteOpts>,
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
            let Some(opts) = self.overwite_opts else {
                println!("The gitignore file already exists in your current directory");
                println!("Please elect to either overwrite, append or exit");
                anyhow::bail!("");
            };

            if opts.no_overwrite {
                println!("Goodbye!");
                return Ok(());
            }
            if opts.append {
                // Append written content to the end of the existing file
                openopts.append(true);
            }
            if opts.overwrite {
                openopts.write(true);
                openopts.truncate(true);
            }
        }

        let mut file = openopts.open(&path)?;

        println!("Getting template {}", template.name);
        writeln!(file, "# {}", template.file_name)?;
        write!(file, "{}", template.contents)?;

        Ok(())
    }
}
