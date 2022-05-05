#[macro_export]
macro_rules! flush_stdout {
    () => {
        io::stdout()
            .flush()
            .with_context(|| "Failed to flush stdout")?
    };
}

#[macro_export]
macro_rules! sleep_for {
    ($time:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($time));
    };
}

#[macro_export]
macro_rules! parse_url {
    ($url:tt) => {
        format!(
            "https://raw.githubusercontent.com/github/gitignore/main/{}.gitignore",
            $url
        )
    };
}
