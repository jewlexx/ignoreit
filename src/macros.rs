//! Global macros

/// Pause execution for given amount of time in milliseconds
#[macro_export]
macro_rules! sleep_for {
    ($time:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($time));
    };
}

/// Parse gitignore download URL
#[macro_export]
macro_rules! parse_url {
    ($url:tt) => {
        format!("https://www.toptal.com/developers/gitignore/api/{}", $url)
    };
}
