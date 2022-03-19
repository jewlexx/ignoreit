use spinners::{Spinner, Spinners};

use crate::lib::get_templates;

pub fn list_templates() -> anyhow::Result<()> {
    let sp = Spinner::new(Spinners::Dots12, "Fetching templates...".into());
    let map = get_templates();
    sp.stop();

    println!("Available templates:");

    for item in map.values() {
        println!("  {}", item);
    }

    println!("\nEnter one of the above names eg. Rust");

    Ok(())
}
