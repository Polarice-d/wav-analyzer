mod encoder;
mod decoder;
mod types;
mod effect_parser;
mod effect_modules;

use std::{collections::HashMap, path::PathBuf};
use clap::{CommandFactory, Parser, error::ErrorKind};
use std::time::Instant;

use crate::types::AudioEffect;

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

    /// Set a fixed tail duration in seconds 
    #[arg(long, short)]
    tail: Option<f32>,

    /// The effects chain
    effects: Vec<String>,
}

fn error(message: String, kind: ErrorKind) {
    let mut cmd = Args::command();
    cmd.error(kind, message).exit();
}

fn add_effect<T: AudioEffect + 'static>(effect: T, effect_map: &mut HashMap<String, Box<dyn AudioEffect>>) {
    effect_map.insert(effect.get_name(), Box::new(effect));
}

fn main() {
    let time = Instant::now();
    let mut effect_map: HashMap<String, Box<dyn AudioEffect>> = HashMap::new();

    // HERE IS WHERE YOU ADD EFFECTS --> //
    add_effect(effect_modules::delay::Delay, &mut effect_map);
    add_effect(effect_modules::gain::Gain, &mut effect_map);
    add_effect(effect_modules::softclip::Softclip, &mut effect_map);
    add_effect(effect_modules::normalize::Normalize, &mut effect_map);

    // <-- HERE IS WHERE YOU ADD EFFECTS//

    let args = Args::parse();
    if args.output.is_some() && args.overwrite {
        error("Cannot use output (-o) and overwrite (--overwrite) at the same time".to_string(), ErrorKind::ArgumentConflict);
        return;
    }

    if args.output.is_none() && !args.overwrite {
        error("Must specify either output (-o) or overwrite (--overwrite)".to_string(), ErrorKind::MissingRequiredArgument);
        return;
    }

    let effect_chain = match effect_parser::parse_effects(&args.effects) {
        Ok(v) => v,
        Err(message) => {
            error(message, ErrorKind::InvalidValue);
            return;
        }
    };

    for effect_spec in effect_chain.iter() {
        match effect_map.get(&effect_spec.name) {
            Some(effect) => {
                match effect.validate_arguments(&effect_spec.arguments, &args.tail) {
                    Ok(_) => {},
                    Err(message) => {
                        error(message, ErrorKind::InvalidValue);
                        return;
                    }
                }
            },
            None => {
                error(format!("unknown effect '{}'", effect_spec.name), ErrorKind::UnknownArgument);
                return;
            }
        }
    }

    let mut buffer = match decoder::read_and_normalize_wav(&args.file_path) {
        Ok(val) => val,
        Err(e) => {error(e, ErrorKind::Io); return;}
    };

    for effect_spec in effect_chain.iter() {
        let effect = effect_map.get(&effect_spec.name).unwrap();
        println!("Applying effect '{}'", effect_spec.name);
        match effect.apply_effect(&mut buffer, &effect_spec.arguments, &args.tail) {
            Ok(_) => {},
            Err(message) => {
                error(message, ErrorKind::Io);
            }
        }
    }

    println!("Finished processing, writing to file...\n");

    if args.overwrite {
        encoder::encode_file(buffer, args.file_path);
        return;
    }
    encoder::encode_file(buffer, args.output.unwrap());

    println!("Total processing time: {:.2?}", time.elapsed());
}
