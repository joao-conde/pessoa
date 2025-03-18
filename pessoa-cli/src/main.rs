use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(version)]
struct Args {
    /// Output destination (stdout by default)
    #[arg(short, long, value_name = "FILE")]
    out: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
