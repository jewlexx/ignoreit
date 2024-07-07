use std::{sync::Arc, time::Duration};

use clap::Parser;
use indicatif::ProgressBar;
use tokio::sync::Mutex;

mod clone;
mod config;
mod dirs;

const GITIGNORE_REPO_URL: &str = "https://github.com/github/gitignore";

#[derive(Debug, Clone, Parser)]
struct Args {
    #[cfg(debug_assertions)]
    #[clap(short, long, help = "Debug first run")]
    debug_first_run: bool,
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

    let config = config::Config::load()?;
    let config = Arc::new(Mutex::new(config));

    #[cfg(debug_assertions)]
    if args.debug_first_run {
        config.lock().await.first_run = true;

        std::fs::remove_dir_all(dirs::templates_dir().unwrap()).unwrap();
    }

    let background_task = tokio::spawn({
        let config = config.clone();
        async move {
            if config.lock().await.first_run {
                return;
            }

            // TODO: Check for updates and update if necessary
        }
    });

    if config.lock().await.first_run {
        background_task.await?;

        // Lock config after background task has finished
        let mut config = config.lock().await;

        config.first_run = false;
        config.save()?;

        println!("Cloning templates...");
        println!("This will only happen once");

        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(100));
        clone::clone(GITIGNORE_REPO_URL, &dirs::templates_dir().unwrap()).unwrap();
    }

    Ok(())
}
