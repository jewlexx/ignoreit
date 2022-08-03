use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use anyhow::Context;

use crate::{
    cache::{get_template, get_templates},
    commands::args::PullOpts,
};

pub fn pull_template(
    output: &str,
    template: Option<String>,
    append: &bool,
    overwrite: &bool,
    no_overwrite: &bool,
) -> anyhow::Result<()> {
    let template_name = template
        .or_else(|| {
            use dialoguer::{theme::ColorfulTheme, Select};

            let values = {
                let map = match get_templates() {
                    Ok(v) => v,
                    Err(_) => return None,
                };

                let mut values = map.values().cloned().collect::<Vec<String>>();
                values.sort();

                values
            };

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose one of the following templates")
                .items(values.as_slice())
                .default(0)
                .interact();

            match selection {
                Ok(v) => values.get(v).cloned(),
                Err(_) => None,
            }
        })
        .context("Failed to get template. Please double check your input")?;

    let template_map = get_templates()?;

    let template_path = template_map
        .get(&template_name.to_lowercase())
        .with_context(|| "Template not found")?;

    let path = env::current_dir()
        .with_context(|| "Failed to get current directory")?
        .join(output);

    let contents = {
        let mut contents = String::new();

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

                    let a = match selection {
                        0 => PullOpts::Overwrite,
                        1 => PullOpts::Append,
                        // 2 and anything else
                        _ => PullOpts::NoOverwrite,
                    };

                    Ok(a)
                })?;

            if opt == PullOpts::NoOverwrite {
                println!("Goodbye!");
                return Ok(());
            } else if opt == PullOpts::Append {
                let mut file = File::open(&path).with_context(|| "Failed to open file")?;

                file.read_to_string(&mut contents)
                    .with_context(|| "Failed to read file")?;
            }
        }

        if *crate::utils::CACHE_ENABLED {
            println!("Getting template {}", template_path);
            let template = get_template(template_path)?;
            let title = format!("# {}.gitignore\n", template_name);
            contents.push_str(&title);
            contents.push_str(std::str::from_utf8(&template)?);
        }

        contents
    };

    let mut file = File::create(path).with_context(|| "Failed to create file")?;
    file.write_all(contents.as_bytes())
        .with_context(|| "Failed to write to file")?;

    Ok(())
}
