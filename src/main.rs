use std::sync::Arc;

use tokio::sync::Mutex;

mod clone;
mod config;
mod dirs;

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
    let config = config::Config::load()?;
    let config = Arc::new(Mutex::new(config));

    tokio::spawn({
        let config = config.clone();
        async move {
            if config.lock().await.first_run {
                return;
            }

            // TODO: Check for updates and update if necessary
        }
    });

    let cache_dir = dirs::cache_dir().unwrap();

    if config.lock().await.first_run {
        clone::clone("https://github.com/jewlexx/ignoreit", "/tmp/ignoreit").unwrap();
    }

    Ok(())
}
