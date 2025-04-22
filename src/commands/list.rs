use crate::cache::CacheHandler;

pub fn run(cache: &CacheHandler) -> anyhow::Result<()> {
    let templates = cache.get_template_paths()?;

    println!("Available templates:");

    for item in templates {
        println!("  {}", item);
    }

    println!("\nEnter one of the above names eg. Rust");

    Ok(())
}
