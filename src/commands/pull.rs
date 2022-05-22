use std::{
    env,
    fs::File,
    io::{self, Read, Write},
};

use anyhow::Context;
use colored::Colorize;

use crate::{
    cache::{get_template, get_templates},
    commands::args::PullOpts,
    flush_stdout,
};

use super::args::ARGS;

pub fn pull_template() -> anyhow::Result<()> {
    let template_name = env::args()
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

    let output = ARGS.output.clone().unwrap_or_else(|| ".gitignore".into());

    let template_map = get_templates()?;

    let template_path = template_map
        .get(&template_name.to_lowercase())
        .with_context(|| "Template not found")?;

    let path = env::current_dir()
        .with_context(|| "Failed to get current directory")?
        .join(output);

    let contents = {
        use crate::utils::CACHE_ENABLED;

        let mut contents = String::new();

        if path.exists() {
            let opt = ARGS.pull_opt.unwrap_or_else(|| {
                print!(
                    "{} already exists. What would you like to do? ({o}verwrite/{a}ppend/{e}xit)",
                    path.display(),
                    o = "O".underline(),
                    a = "A".underline(),
                    e = "E".underline()
                );

                flush_stdout!().unwrap();

                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .with_context(|| "Failed to read input")
                    .unwrap();

                let answer = input
                    .trim()
                    .chars()
                    .next()
                    .context("invalid input")
                    .unwrap()
                    .to_lowercase()
                    .to_string();

                match answer.as_str() {
                    "o" => PullOpts::Overwrite,
                    "a" => PullOpts::Append,
                    "e" => PullOpts::NoOverwrite,
                    _ => PullOpts::NoOverwrite,
                }
            });

            if opt == PullOpts::NoOverwrite {
                return Ok(());
            } else if opt == PullOpts::Append {
                let mut file = File::open(&path).with_context(|| "Failed to open file")?;

                file.read_to_string(&mut contents)
                    .with_context(|| "Failed to read file")?;
            }
        }

        if CACHE_ENABLED.to_owned() {
            println!("Getting template {}", template_path);
            let template = get_template(template_path)?;
            let title = format!("\n# {}.gitignore\n", template_name);
            contents.push_str(&title);
            contents.push_str(template.as_str());
        }

        contents
    };

    let mut file = File::create(path).with_context(|| "Failed to create file")?;
    file.write_all(contents.as_bytes())
        .with_context(|| "Failed to write to file")?;

    Ok(())
}
