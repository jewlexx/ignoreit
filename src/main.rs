use std::{sync::Arc, time::Duration};

use clap::Parser;
use indicatif::ProgressBar;
use tokio::sync::Mutex;

mod cache;
mod clone;
mod commands;
mod config;
mod dirs;
pub mod progress;
mod template;

#[derive(Debug, Clone, Parser)]
struct Args {
    #[clap(subcommand)]
    command: commands::Commands,

    #[cfg(debug_assertions)]
    #[clap(long, help = "Debug first run")]
    debug_first_run: bool,

    #[clap(short, long, help = "Dry run", global = true)]
    dry_run: bool,
}

fn main() {
    if let Err(e) = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Correctly initialized asynchronous runtime")
        .block_on(_main())
    {
        // TODO: Better error handling
        panic!("{:?}", e)
    }
}

async fn _main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = Arc::new(Mutex::new(config::Config::load()?));

    #[cfg(debug_assertions)]
    if args.debug_first_run {
        config.lock().await.first_run = true;

        std::fs::remove_dir_all(cache::Cache::path().unwrap()).unwrap();
    }

    let first_run = config.lock().await.first_run;

    // let file_loading = CounterProgress::new(, callback)

    let cache = if first_run {
        // Lock config after background task has finished
        let mut config = config.lock().await;

        config.first_run = false;
        config.save()?;

        println!("Cloning templates...");
        println!("This will only happen once");

        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(100));
        cache::Cache::clone()?
    } else {
        let pb = ProgressBar::new_spinner().with_message("Loading templates...");
        pb.enable_steady_tick(Duration::from_millis(100));
        cache::Cache::open()?
    };

    let cache = Arc::new(cache);

    let background_task = tokio::spawn({
        let cache = cache.clone();
        let config = config.clone();
        async move {
            if first_run {
            } else if cache.open_repo().unwrap().outdated().unwrap() {
                // TODO: Update cache
            }

            // TODO: Check for updates and update if necessary
        }
    });

    args.command.run()?;

    background_task.await?;

    Ok(())
}
