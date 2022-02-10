use reqwest::{
    blocking::{Client, Response},
    header::USER_AGENT,
};
use serde_json::Value;
use std::{env, fs::File, io, io::Write, path::PathBuf,collections::HashMap};

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

let mut templates_collection = HashMap::new();

fn get_templates(client: &Client) -> Value {
    let templates_url = "https://api.github.com/repos/github/gitignore/git/trees/main";

    let templates = get_url(templates_url, client).json().expect("Failed to read JSON from response");

            let tree = templates["tree"].as_array().unwrap().iter().filter(|el| {
            let name = el["path"].as_str().unwrap();
            name.ends_with(".gitignore")
        });

        tree
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
        path.push(".gitignore");

        if path.exists() {
            print!(
                "{} already exists. Would you like to continue? (y/N)",
                path.display()
            );

            io::stdout().flush().unwrap();

            let mut input = String::new();

            io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() != "y" {
                return;
            }
        }

        let mut file = File::create(path).unwrap();
        file.write(body.as_bytes()).unwrap();
    } else if command == "ls" || command == "list" {
        let templates = get_templates(&client);

        println!("Available templates:");

        for item in templates {
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
