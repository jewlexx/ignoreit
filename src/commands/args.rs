use clap::{Arg, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(short, long, default_value = ".gitignore")]
    pub output: String,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    #[clap(about = "List all available templates")]
    List,
    #[clap(about = "Pull a template from the repository", args = [
        Arg::new("template").takes_value(true).required(true),
        Arg::new("output").takes_value(true).required(false),
    ])]
    Pull,
}

pub fn parse() -> Args {
    Args::parse()
}
