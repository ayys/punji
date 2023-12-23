use clap::Parser;
use manifest::PackageManifest;
use schemars::schema_for;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Specify output file for schema.
    #[arg(short, long, value_name = "FILE", default_value = "schema/punji.json")]
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let outfile = cli.output.unwrap();
    let schema = schema_for!(PackageManifest);
    let json_schema = serde_json::to_string_pretty(&schema).unwrap();
    std::fs::write(outfile, json_schema).unwrap();
}
