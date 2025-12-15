mod encoder;
mod decoder;
mod types;
mod effect_parser;
mod dynamics;

use std::path::PathBuf;
use clap::{CommandFactory, Parser, error::ErrorKind};

#[derive(Parser)]
#[command(name="fiis", version, about, long_about= None)]
struct Args {
    /// File path of the .wav file
    #[arg()]
    file_path: PathBuf,

    /// Where to output the processed file
    #[arg(short, long)]
    output: Option<PathBuf>,
    
    /// Whether to edit the original file
    #[arg(long)]
    overwrite: bool,

    /// The effects chain
    effects: Vec<String>,
}

fn error(message: String, kind: ErrorKind) {
    let mut cmd = Args::command();
    cmd.error(kind, message).exit();
}

fn main() {
    let args = Args::parse();
    let mut buffer = match decoder::read_and_normalize_wav(&args.file_path) {
        Ok(val) => val,
        Err(e) => {error(e, ErrorKind::Io); return;}
    };

    let effect_chain = effect_parser::parse_effects(&args.effects).unwrap();

    for effect in effect_chain.iter() {
        match effect.effect_name.as_str() {
            "gain" => dynamics::gain(&mut buffer, &effect.arguments).unwrap(),
            "softclip" => dynamics::distortion(&mut buffer, &effect.arguments).unwrap(),
            _ => {panic!("Uknown effect")}
        }
    }

    if args.overwrite {
        encoder::encode_file(buffer, args.file_path);
    } else if args.output.is_some() {
        encoder::encode_file(buffer, args.output.unwrap());
    } else {
        error("no output file specified (use --overwrite to edit the original file)".into(), ErrorKind::MissingRequiredArgument);
    }
}
