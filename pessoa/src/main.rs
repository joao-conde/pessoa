use clap::Parser;
use pessoa::create_identity;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
struct Args {
    /// Output format
    #[arg(long)]
    file: Option<PathBuf>,

    /// With email
    #[arg(long)]
    with_email: bool,

    /// With phone
    #[arg(long)]
    with_phone: bool,
}

fn main() {
    let args = Args::parse();
    println!("{args:#?}");

    let identity = create_identity();
    println!("{identity:#?}");
}
