//! CLI Commands

pub mod args;
mod list;
mod pull;

/// List all templates and write to stdout
pub fn list_templates() -> anyhow::Result<()> {
    list::run()
}

/// Pull a given template
pub fn pull_template(
    output: &str,
    template: Option<String>,
    append: &bool,
    overwrite: &bool,
    no_overwrite: &bool,
) -> anyhow::Result<()> {
    pull::run(output, template, append, overwrite, no_overwrite)
}
