use anyhow::Result;
use reqwest::blocking::Client;
use spinners::{Spinner, Spinners};

use crate::lib::get_templates;

pub fn list_templates() -> Result<()> {
    let client = Client::new();

    let sp = Spinner::new(Spinners::Dots12, "Fetching templates...".into());
    let map = get_templates(&client);
    sp.stop();

    println!("Available templates:");

    for item in map.values() {
        println!("  {}", item);
    }

    println!("\nEnter one of the above names eg. Rust");

    Ok(())
}
