use reqwest::{
    blocking::{Client, Response},
    header::USER_AGENT,
};
use serde_json::Value;
use std::{env, fs::File, io::Write, path::PathBuf};

fn get_url(str: &str, client: &Client) -> Response {
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

fn get_templates(client: &Client) -> Value {
    let templates_url = "https://api.github.com/repos/github/gitignore/git/trees/main";

    let body = get_url(templates_url, client);

    body.json().expect("Failed to read JSON from response")
}

fn main() {
    let client = Client::new();

    let mut args: env::Args = env::args();
    let command = args.nth(1).expect("No command given");

    if command == "pull" {
        let mut template = args.nth(0).expect("No template given");
        template = template
            .split('.')
            .nth(0)
            .expect("No template given")
            .to_string();

        let url = format!(
            "https://raw.githubusercontent.com/github/gitignore/main/{}.gitignore",
            template
        );

        let body = get_url(&url, &client)
            .text()
            .expect("Failed to read text from response");

        let mut path = PathBuf::from(env::current_dir().unwrap());
        path.push("Test.gitignore");

        let mut file = File::create(path).unwrap();
        file.write(body.as_bytes()).unwrap();
    } else if command == "ls" || command == "list" {
        let templates = get_templates(&client);

        let tree = templates["tree"].as_array().unwrap().iter().filter(|el| {
            let name = el["path"].as_str().unwrap();
            name.ends_with(".gitignore")
        });

        println!("Available templates:");

        for item in tree {
            let name = item["path"]
                .as_str()
                .unwrap()
                .split('.')
                .nth(0)
                .unwrap()
                .to_string();

            println!("  {}", name);
        }

        println!("\nEnter one of the above names. Example: Rust");
        println!("NOTE: CASE SENSITIVE. Working on this :)");
        println!("These are simply the Github templates. If you would like a different one, look elsewhere.");
    }
}
