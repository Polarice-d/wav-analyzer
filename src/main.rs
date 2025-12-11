use std::path::PathBuf;

use hound;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
struct Args {
    /// File path of the WAV file
    #[arg(short, long, value_name = "FILE")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let mut reader = hound::WavReader::open(args.path.as_path()).unwrap();
    let num = reader.samples::<i16>().count(); // I think this is the number of samples but i'm not sure
    println!("{num}");
}
