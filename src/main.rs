mod commands;
mod lib;
mod remote;

#[macro_use]
mod macros;

#[cfg(feature = "cache")]
mod cache;

use commands::{
    args::{parse, Commands},
    list::list_templates,
    pull::pull_template,
};

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "cache")]
    {
        if lib::CACHE_ENABLED.to_owned() {
            cache::init_cache()?;
        } else {
            use colored::Colorize;
            if !lib::IS_ONLINE.to_owned() {
                println!(
                    "{}",
                    "error: you are offline and cache is disabled. we cannot continue".red()
                );
                std::process::exit(1);
            }

            println!(
                "{}",
                "warning: cache is disabled. performance will not be optimal".yellow()
            );
            sleep_for!(3000);
        }
    }

    let args = parse()?;

    println!("{:?}", args.command);

    match args.command {
        Commands::List => list_templates()?,
        Commands::Pull => pull_template()?,
        #[cfg(feature = "cache")]
        Commands::Purge => cache::purge()?,
    }

    Ok(())
}
