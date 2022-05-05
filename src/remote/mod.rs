use anyhow::Context;
use reqwest::{blocking::Response, header::USER_AGENT};

use crate::create_client;

pub fn get_url(str: &str) -> anyhow::Result<Response> {
    let client = create_client!();

    let res = client
        .get(str)
        .header(USER_AGENT, "Gitignore Generator")
        .send()
        .with_context(|| "Failed to send request")?;

    if !res.status().is_success() {
        panic!("Failed to get response: {}", res.status())
    }

    Ok(res)
}
