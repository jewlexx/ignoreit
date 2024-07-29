mod list;
mod pull;
mod purge;

pub trait Command {
    const INTERRUPT_BACKGROUND_TASK: bool = false;

    fn interrupt_background_task(&self) -> bool {
        Self::INTERRUPT_BACKGROUND_TASK
    }

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
    pub fn interrupt_background_task(&self) -> bool {
        match self {
            Commands::List(args) => args.interrupt_background_task(),
            Commands::Pull(args) => args.interrupt_background_task(),
            Commands::Purge(args) => args.interrupt_background_task(),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Commands::List(args) => args.run().await,
            Commands::Pull(args) => args.run().await,
            Commands::Purge(args) => args.run().await,
        }
    }
}
