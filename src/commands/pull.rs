use std::{
    env,
    fs::File,
    io::{self, Write},
    time::SystemTime,
};

use anyhow::Context;

use crate::{
    flush_stdout,
    lib::{get_templates, CACHE_DIR, IS_ONLINE},
};

pub fn get_contents_remote(template_path: &str) -> anyhow::Result<Vec<u8>> {
    let url = crate::parse_url!(template_path);
    let contents = crate::remote::get_url(&url)?.text()?.as_bytes().to_vec();

    #[cfg(feature = "cache")]
    {
        let cache_file = CACHE_DIR
            .to_owned()
            .context("Failed to find cache dir")?
            .join(format!("{}.gitignore", template_path));

        let mut file = File::create(cache_file)?;

        file.write_all(&contents)?;
    }

    Ok(contents)
}

pub fn pull_template() -> anyhow::Result<()> {
    let template = env::args()
        .nth(2)
        .with_context(|| "Please provide a template name")?;

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

    #[cfg(feature = "cache")]
    let contents = {
        use crate::lib::CACHE_ENABLED;

        if CACHE_ENABLED.to_owned() {
            let cache_dir = CACHE_DIR.to_owned().context("Failed to parse cache dir")?;
            let file_path = cache_dir.join(format!("{}.gitignore", template_path));
            let last_modified = file_path.metadata()?.modified()?;

            if SystemTime::now().duration_since(last_modified)?.as_secs() * 60 * 60 > 24
                && IS_ONLINE.to_owned()
            {
                println!("Getting template {}", template_path);
                get_contents_remote(template_path)?
            } else {
                match std::fs::read(file_path) {
                    Ok(v) => v,
                    Err(_) => get_contents_remote(template_path)?,
                }
            }
        } else {
            Vec::new()
        }
    };

    #[cfg(not(feature = "cache"))]
    let contents = get_contents_remote(template_path)?;

    let mut file = File::create(path).with_context(|| "Failed to create file")?;
    file.write_all(&contents)
        .with_context(|| "Failed to write to file")?;

    Ok(())
}
