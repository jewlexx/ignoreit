use std::{
    sync::{LazyLock, OnceLock},
    time::Duration,
};

use clap::Parser;
use indicatif::ProgressBar;
use once::UnsafeOnce;
use tokio::sync::Mutex;

mod cache;
mod clone;
mod commands;
mod config;
mod dirs;
mod once;
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

static IS_TERMINAL: LazyLock<bool> =
    LazyLock::new(|| std::io::IsTerminal::is_terminal(&std::io::stdout()));
static CONFIG: UnsafeOnce<Mutex<config::Config>> = UnsafeOnce::new();
static CACHE: UnsafeOnce<cache::Cache> = UnsafeOnce::new();

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

    CONFIG
        .set(Mutex::new(config::Config::load()?))
        .expect("Correctly initialized config");

    #[cfg(debug_assertions)]
    if args.debug_first_run {
        CONFIG.lock().await.first_run = true;

        std::fs::remove_dir_all(cache::Cache::path().unwrap()).unwrap();
    }

    let first_run = CONFIG.lock().await.first_run;

    // let file_loading = CounterProgress::new(, callback)

    let cache = if first_run {
        // Lock config after background task has finished
        let mut config = CONFIG.lock().await;

        config.first_run = false;
        config.save()?;

        drop(config);

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

    CACHE.set(cache).expect("Correctly initialized cache");

    let background_task = if !args.command.interrupt_background_task() {
        Some(tokio::spawn({
            async move {
                if first_run {
                    return;
                }

                if CACHE.open_repo().unwrap().outdated().unwrap() {
                    // TODO: Update cache
                }
            }
        }))
    } else {
        None
    };

    args.command.run().await?;

    if let Some(task) = background_task {
        task.await?;
    };

    Ok(())
}
