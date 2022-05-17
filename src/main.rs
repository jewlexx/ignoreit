mod cache;
mod commands;
mod lib;
mod sys;

#[macro_use]
mod macros;

use commands::args::ARGS;

fn main() -> anyhow::Result<()> {
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

    if let Some(cmd) = ARGS.command {
        cmd.run()?;
    }

    Ok(())
}
