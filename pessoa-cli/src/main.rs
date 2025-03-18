use clap::Parser;
use pessoa::{Identity, Locale};
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Debug, Clone, Parser)]
#[command(version)]
struct Args {
    /// Output destination (stdout by default)
    #[arg(short, long, value_name = "FILE")]
    out: Option<PathBuf>,

    /// Locale used for identity data (e.g. en_us)
    #[arg(short, long)]
    locale: Locale,
}

fn main() {
    let args = Args::parse();

    let identity = Identity::builder().build();

    let json = serde_json::to_string_pretty(&identity).unwrap();
    if let Some(path) = args.out {
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    } else {
        println!("{json}");
    }
}
