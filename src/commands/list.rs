use crate::cache::get_templates;

pub fn list_templates() -> anyhow::Result<()> {
    let map = get_templates()?;

    println!("Available templates:");

    let mut values = map.values().collect::<Vec<&String>>();
    values.sort();

    for item in values {
        println!("  {}", item);
    }

    println!("\nEnter one of the above names eg. Rust");

    Ok(())
}
