#[macro_export]
macro_rules! flush_stdout {
    () => {{
        io::stdout()
            .flush()
            .with_context(|| "Failed to flush stdout")?;
    }};
}

#[macro_export]
macro_rules! create_client {
    () => {{
        use reqwest::blocking::Client;

        Client::new()
    }};
}

#[macro_export]
macro_rules! sleep_for {
    ($time:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($time));
    };
}