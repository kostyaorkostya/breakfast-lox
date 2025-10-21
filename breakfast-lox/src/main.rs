use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        /// Script to execute
        script: Option<PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;

    let _script = match args.command {
        Commands::Run { script } => {
            let mut reader: Box<dyn BufRead> = match script {
                None => Box::new(BufReader::new(std::io::stdin())),
                Some(path) => Box::new(BufReader::new(File::open(path)?)),
            };
            let mut contents = String::new();
            reader.read_to_string(&mut contents)?;
            contents
        }
    };

    Ok(())
}
