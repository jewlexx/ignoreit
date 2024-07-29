mod list;
mod pull;
mod purge;

pub trait Command {
    async fn run(&self) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Commands {
    #[clap(about = "List all available templates")]
    List(list::Args),
    #[clap(about = "Pull a template from the repository")]
    Pull(pull::Args),
    #[clap(about = "Purge gitignore cache")]
    Purge(purge::Args),
}

impl Commands {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Commands::List(args) => args.run().await,
            Commands::Pull(args) => args.run().await,
            Commands::Purge(args) => args.run().await,
        }
    }
}
