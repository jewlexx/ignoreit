//! CLI Commands

use crate::cache::CacheHandler;

pub mod args;
mod list;
mod pull;

/// List all templates and write to stdout
pub fn list_templates(cache: &CacheHandler) -> anyhow::Result<()> {
    list::run(cache)
}

/// Pull a given template
pub fn pull_template(
    cache: &CacheHandler,
    output: &str,
    template: Option<String>,
    append: &bool,
    overwrite: &bool,
    no_overwrite: &bool,
) -> anyhow::Result<()> {
    pull::run(cache, output, template, append, overwrite, no_overwrite)
}
