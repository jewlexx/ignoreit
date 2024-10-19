use std::sync::LazyLock;

use clap::Parser;

mod commands;
mod templates;

#[derive(Debug, Clone, Parser)]
struct Args {
    #[clap(subcommand)]
    command: commands::Commands,

    #[cfg(debug_assertions)]
    #[clap(long, help = "Debug first run")]
    debug_first_run: bool,

    #[clap(short, long, help = "Dry run", global = true)]
    dry_run: bool,
}

static IS_TERMINAL: LazyLock<bool> =
    LazyLock::new(|| std::io::IsTerminal::is_terminal(&std::io::stdout()));

fn main() {
    if let Err(e) = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Correctly initialized asynchronous runtime")
        .block_on(_main())
    {
        // TODO: Better error handling
        panic!("{:?}", e)
    }
}

async fn _main() -> anyhow::Result<()> {
    let args = Args::parse();

    args.command.run().await?;

    Ok(())
}
