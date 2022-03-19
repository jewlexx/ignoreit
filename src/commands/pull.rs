use anyhow::Result;
use reqwest::blocking::Client;
use std::{
    env,
    fs::File,
    io::{self, Write},
};

use crate::lib::{get_templates, get_url};

pub fn pull_template() -> Result<()> {
    let client = Client::new();

    let template = env::args().nth(3).unwrap();
    let template_map = get_templates(&client);

    let template_path = template_map
        .get(&template.to_lowercase())
        .expect("Failed to find template in collection");

    let url = format!(
        "https://raw.githubusercontent.com/github/gitignore/main/{}.gitignore",
        template_path
    );

    let body = get_url(&url, &client)
        .text()
        .expect("Failed to read text from response");

    let mut path = env::current_dir().unwrap();
    path.push(".gitignore");

    if path.exists() {
        print!(
            "{} already exists. Would you like to continue? (y/N)",
            path.display()
        );

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() != "y" {
            return Ok(());
        }
    }

    let mut file = File::create(path).unwrap();
    file.write_all(body.as_bytes()).unwrap();

    Ok(())
}
