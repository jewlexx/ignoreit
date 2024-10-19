use std::path::PathBuf;

#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    #[clap(
        short,
        long,
        help = "The path to save the template at",
        default_value = ".gitignore"
    )]
    output: PathBuf,

    #[clap(help = "Template name to pull")]
    template_name: Option<String>,

    #[clap(short, long, help = "Append to the end of the file")]
    append: bool,
}

impl super::Command for Args {
    async fn run(&self) -> anyhow::Result<()> {
        println!("Loading template...");

        // let template = unimplemented!();

        // let mut file = File::open(template.path())?;
        // let mut destination = OpenOptions::new()
        //     .create(true)
        //     .write(true)
        //     .append(self.append)
        //     .open(&self.output)?;

        // std::io::copy(&mut file, &mut destination)?;

        // println!("Template saved to {}", self.output.display());

        Ok(())
    }
}
