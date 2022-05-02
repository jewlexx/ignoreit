use clap::{Arg, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
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

    #[cfg(feature = "cache")]
    #[clap(about = "Purge gitignore cache")]
    Purge,
}
