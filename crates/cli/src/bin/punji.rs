use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, WarnLevel};
use log::{info, warn};
use manifest::{Project};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[command(flatten)]
    verbose: Verbosity<WarnLevel>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Build {
        /// Input file to parse
        #[clap(default_value = "punji.yaml")]
        input: PathBuf,

        /// overwrite files if they exist
        #[arg(short, long, default_value = "false")]
        overwrite: bool,
    },
}

fn write_to_file<T>(document: T, overwrite: bool) -> anyhow::Result<()>
where
    T: manifest::convert::ConvertToFile,
{
    // check if file already exists
    let filename = document.filename();
    if filename.exists() {
        if overwrite {
            warn!(
                "File already exists: `{}` Overwriting...",
                filename.display()
            );
        } else {
            return Err(anyhow::anyhow!(
                "File already exists: `{}`\nUse --overwrite to overwrite the file(s).",
                filename.display()
            ));
        }
    }
    std::fs::write(filename, document.convert().unwrap()).unwrap();
    info!("Created: {}", filename.display());
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(cli.verbose.log_level_filter())
        .try_init()?;

    match cli.command {
        Commands::Build { input, overwrite } => {
            let contents = std::fs::read_to_string(input).unwrap();
            let project = Project::parse_many(&contents).unwrap();
            for document in project {
                match document {
                    Project::Package(package) => write_to_file(package.as_ref(), overwrite)?,
                    Project::App(app) => write_to_file(app, overwrite)?,
                }
            }
        }
    }
    Ok(())
}
