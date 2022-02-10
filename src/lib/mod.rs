use reqwest::{
    blocking::{Client, Response},
    header::USER_AGENT,
};
use serde_json::Value;
use std::collections::HashMap;

pub fn get_url(str: &str, client: &Client) -> Response {
    let res = client
        .get(str)
        .header(USER_AGENT, "Gitignore Generator")
        .send()
        .expect("Failed to send HTTP request");

    if !res.status().is_success() {
        panic!("Failed to get response: {}", res.status())
    }

    res
}

pub fn get_templates(client: &Client) -> (Value, HashMap<String, String>) {
    let mut hashmap: HashMap<String, String> = HashMap::new();

    let templates_url = "http://api.github.com/repos/github/gitignore/git/trees/main";

    let body: Value = get_url(templates_url, client)
        .json()
        .expect("Failed to read JSON from response");

    let tree = body["tree"].as_array().unwrap().iter().filter(|el| {
        let name = el["path"].as_str().unwrap();
        name.ends_with(".gitignore")
    });

    for item in tree {
        let base_path = item["path"].as_str().unwrap();
        let path = base_path.split(".").nth(0).unwrap();
        let lowercase = &path.to_lowercase();

        println!("{}", &lowercase);
        println!("{}", &path);

        hashmap.insert(lowercase.to_string(), path.to_string());
    }

    (body, hashmap)
}
