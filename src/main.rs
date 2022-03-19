mod commands;
mod lib;

use commands::{
    args::{parse, Commands},
    list::list_templates,
    pull::pull_template,
};

fn main() -> anyhow::Result<()> {
    let args = parse();

    if args.command == Commands::List {
        list_templates()?
    } else if args.command == Commands::Pull {
        pull_template()?
    }

    Ok(())
}
