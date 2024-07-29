use std::{fmt::Write, io::Write as _};

use crate::CACHE;

struct NoPager;

impl std::fmt::Write for NoPager {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        std::io::stdout().write_all(s.as_bytes()).unwrap();

        Ok(())
    }
}

enum Output {
    Pager(minus::Pager),
    NoPager,
}

impl Output {
    fn page(self) -> Result<(), minus::MinusError> {
        match self {
            Output::Pager(pager) => minus::page_all(pager),
            Output::NoPager => Ok(()),
        }
    }
}

impl std::fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self {
            Output::Pager(pager) => pager.write_str(s)?,
            Output::NoPager => print!("{s}"),
        };

        Ok(())
    }
}

#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    #[clap(short = 'P', long, help = "Disable the paging of the output")]
    no_paging: bool,
}

impl super::Command for Args {
    async fn run(&self) -> anyhow::Result<()> {
        let mut output = if self.no_paging || !*crate::IS_TERMINAL {
            Output::NoPager
        } else {
            Output::Pager(minus::Pager::new())
        };

        writeln!(output, "Available Templates:")?;

        for template in CACHE.list_templates() {
            writeln!(output, "\t{}", template.name())?;
        }

        output.page()?;

        Ok(())
    }
}
