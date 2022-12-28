use std::{
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use anyhow::Context;

use crate::{
    cache::{get_template, get_template_paths},
    commands::args::PullOpts,
};

pub fn run(
    output: &str,
    template: Option<String>,
    append: &bool,
    overwrite: &bool,
    no_overwrite: &bool,
) -> anyhow::Result<()> {
    let template_paths = get_template_paths();

    let template_name = template
        .or_else(|| {
            use dialoguer::{theme::ColorfulTheme, Select};

            let values = match template_paths {
                Ok(v) => v,
                Err(_) => return None,
            };

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose one of the following templates")
                .items(values.as_slice())
                .default(0)
                .interact();

            match selection {
                Ok(v) => values.get(v).map(|x| x.to_string()),
                Err(_) => None,
            }
        })
        .context("Failed to get template. Please double check your input")?;

    let template_map = get_template_paths()?;

    let template_path = if let Some(v) = template_map.iter().find(|f| f.lower == template_name) {
        v
    } else {
        return Err(anyhow::anyhow!("Template not found: {}", template_name));
    };

    let path = env::current_dir()
        .with_context(|| "Failed to get current directory")?
        .join(output);

    let mut openopts = OpenOptions::new();

    if path.exists() {
        let pull_opt = PullOpts::get_opt(append, overwrite, no_overwrite);
        let opt = pull_opt
            .map(anyhow::Ok)
            .unwrap_or_else(|| -> anyhow::Result<PullOpts> {
                use dialoguer::{theme::ColorfulTheme, Select};

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("The gitignore file already exists in your current directory")
                    .items(&["Append", "Overwrite", "Exit"])
                    .default(0)
                    .interact()?;

                Ok(match selection {
                    1 => PullOpts::Append,
                    0 => PullOpts::Overwrite,
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
                // [`OpenOptions::append`] implies [`OpenOptions::write`] so we only have to
                // explicitly write it here
                openopts.write(true);
            }
        }
    }

    let mut file = openopts.open(&path)?;

    if *crate::cache::CACHE_ENABLED {
        println!("Getting template {}", template_path);
        let template = get_template(template_path)?;
        writeln!(file, "# {}.gitignore", template_path)?;
        write!(file, "{}", String::from_utf8(template)?)?;
    }

    Ok(())
}
