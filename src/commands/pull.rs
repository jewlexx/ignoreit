use std::{
    env,
    fs::File,
    io::{self, Write},
};

use anyhow::Context;

use crate::{cache::get_template, flush_stdout, lib::get_templates};

pub fn pull_template() -> anyhow::Result<()> {
    let template = env::args()
        .nth(2)
        .or_else(|| -> Option<String> {
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
                .with_prompt("Choose one of the following templates:")
                .items(values.as_slice())
                .default(0)
                .interact();

            match selection {
                Ok(v) => values.get(v).cloned(),
                Err(_) => None,
            }
        })
        .context("Failed to get template. Please double check your input")?;

    let output = env::args().nth(3).unwrap_or_else(|| ".gitignore".into());

    let template_map = get_templates()?;

    let template_path = template_map
        .get(&template.to_lowercase())
        .with_context(|| "Template not found")?;

    let path = env::current_dir()
        .with_context(|| "Failed to get current directory")?
        .join(output);

    if path.exists() {
        print!(
            "{} already exists. Would you like to continue? (y/N)",
            path.display()
        );

        flush_stdout!();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .with_context(|| "Failed to read input")?;

        if input.trim().to_lowercase() != "y" {
            return Ok(());
        }
    }

    let contents = {
        use crate::lib::CACHE_ENABLED;

        if CACHE_ENABLED.to_owned() {
            println!("Getting template {}", template_path);
            get_template(template_path)?
        } else {
            String::new()
        }
    };

    let mut file = File::create(path).with_context(|| "Failed to create file")?;
    file.write_all(contents.as_bytes())
        .with_context(|| "Failed to write to file")?;

    Ok(())
}
