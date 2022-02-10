mod lib;
use lib::{get_templates, get_url};
use reqwest::blocking::Client;
use spinners_rs::{Spinner, Spinners};
use std::{env, fs::File, io, io::Write, path::PathBuf};

fn print_usage() {
    println!(
        "Usage: {} <command>",
        env::args().collect::<Vec<String>>()[0]
    );
    println!("Commands:");
    println!("  list");
    println!("  pull <template>");
}

fn main() {
    let client = Client::new();

    let args = env::args().collect::<Vec<String>>();
    let command = match &args.get(1) {
        Some(arg) => arg.to_string(),
        None => {
            print_usage();
            return;
        }
    };

    let sp = Spinner::new(&Spinners::Dots12, "Fetching templates...".into());
    let (templates, template_map) = get_templates(&client);
    sp.stop();

    if command == "pull" {
        let template = match &args.get(2) {
            Some(arg) => arg.to_string().split('.').nth(0).unwrap().to_string(),
            None => {
                print_usage();
                return;
            }
        };

        let template_path = template_map
            .get(&template.to_lowercase())
            .expect("Failed to find template in collection");

        let url = format!(
            "https://raw.githubusercontent.com/github/gitignore/main/{}.gitignore",
            template_path
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

        println!("\nEnter one of the above names eg. Rust");
        println!("These are simply the Github templates. If you would like a different one, look elsewhere.");
    }
}
