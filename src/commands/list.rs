use crate::lib::get_templates;

pub fn list_templates() -> anyhow::Result<()> {
    let map = get_templates()?;

    println!("Available templates:");

    for item in map.values() {
        println!("  {}", item);
    }

    println!("\nEnter one of the above names eg. Rust");

    Ok(())
}
