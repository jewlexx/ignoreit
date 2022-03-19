use anyhow::Result;
use reqwest::blocking::Client;
use spinners::{Spinner, Spinners};

use crate::lib::get_templates;

pub fn list_templates() -> Result<()> {
    let client = Client::new();

    let sp = Spinner::new(Spinners::Dots12, "Fetching templates...".into());
    let templates = get_templates(&client).0;
    sp.stop();

    let tree = templates["tree"].as_array().unwrap().iter().filter(|el| {
        let name = el["path"].as_str().unwrap();
        name.ends_with(".gitignore")
    });

    println!("Available templates:");

    for item in tree {
        let name = item["path"]
            .as_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap()
            .to_string();

        println!("  {}", name);
    }

    println!("\nEnter one of the above names eg. Rust");
    println!(
        "These are simply the Github templates. If you would like a different one, look elsewhere."
    );

    std::process::exit(0);
}
