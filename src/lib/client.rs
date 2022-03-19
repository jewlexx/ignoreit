#[macro_export]
macro_rules! create_client {
    () => {{
        use reqwest::blocking::Client;

        Client::new()
    }};
}
