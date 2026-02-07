use std::{env, fs::OpenOptions, io::Write};

use anyhow::Context;
use clap::Parser;

use crate::cache::CacheHandler;

#[derive(Debug, Copy, Clone, Parser)]
#[group(multiple = false)]
pub struct OverwriteOpts {
    /// Append the template to the end an existing gitignore
    #[clap(long)]
    append: bool,
    /// Overwrite the template if it already exists
    #[clap(long)]
    overwrite: bool,
}

#[derive(Debug, Parser, Clone)]
pub struct Args {
    /// The name of the template to pull
    template: String,

    /// The path to output the template to
    #[clap(short, long, default_value = ".gitignore")]
    output: String,

    #[clap(flatten)]
    overwite_opts: OverwriteOpts,
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
            if self.overwite_opts.append {
                // Append written content to the end of the existing file
                openopts.append(true);
            } else if self.overwite_opts.overwrite {
                openopts.write(true);
                openopts.truncate(true);
            } else {
                println!("Ignore file already exists.");
                println!("Pass --overwrite or --append to edit existing gitignore,");
                println!("or pass -o <filename> to change the output path.");
                return Ok(());
            }
        }

        let mut file = openopts.open(&path)?;

        println!("Getting template {}", template.name);
        writeln!(file, "# {}", template.file_name)?;
        write!(file, "{}", template.contents)?;

        Ok(())
    }
}
