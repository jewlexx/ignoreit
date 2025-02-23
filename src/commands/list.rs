use crate::cache::get_template_paths;

pub fn run() -> anyhow::Result<()> {
    let templates = get_template_paths()?;

    println!("Available templates:");

    for item in templates {
        println!("  {}", item);
    }

    println!("\nEnter one of the above names eg. Rust");

    Ok(())
}
