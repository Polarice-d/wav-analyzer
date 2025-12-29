mod encoder;
mod decoder;
mod types;
mod audio_utils;
mod parse_utils;
mod effect_modules;

use colored::Colorize;
use std::{collections::HashMap, path::PathBuf, time::Duration};
use clap::{CommandFactory, Parser, error::ErrorKind};
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{audio_utils::get_buffer_duration, types::AudioEffect};

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
    tail: Option<f64>,

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
    add_effect(effect_modules::eq::PeakingEQ, &mut effect_map);
    add_effect(effect_modules::eq::BandPassEQ, &mut effect_map);
    add_effect(effect_modules::eq::HShelfEQ, &mut effect_map);
    add_effect(effect_modules::eq::LShelfEQ, &mut effect_map);
    // <-- HERE IS WHERE YOU ADD EFFECTS//

    let args = Args::parse();
    if args.output.is_some() && args.overwrite {
        error("Cannot use output (-o) and overwrite (--overwrite) at the same time".to_string(), ErrorKind::ArgumentConflict);
        return;
    }

    if args.output.is_none() && !args.overwrite {
        error("No output specified (use --overwrite to replace the original file)".to_string(), ErrorKind::MissingRequiredArgument);
        return;
    }

    let effect_chain = match parse_utils::parse_effects(&args.effects) {
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
                        error(format!("{} -> {message}", effect.get_name()), ErrorKind::InvalidValue);
                        return;
                    }
                }
            },
            None => {
                error(format!("Unknown effect '{}'", effect_spec.name), ErrorKind::UnknownArgument);
                return;
            }
        }
    }

    eprintln!("{}", format!("Reading file {:#?}", &args.file_path).bold());

    let mut buffer = match decoder::read_file(&args.file_path) {
        Ok(val) => val,
        Err(e) => {error(e, ErrorKind::Io); return;}
    };

    let spec = &buffer.spec;

    let message = format!("   Sample rate: {},\n   Duration: {}s,\n   Bit depth: {},\n   Sample format: {},\n   Channels: {}", 
        spec.sample_rate.to_string().bright_blue(),
        format!("{:.2}", audio_utils::get_buffer_duration(&buffer)).bright_blue(),
        spec.bits_per_sample.to_string().bright_blue(),
        format!("{:?}", spec.sample_format).bright_blue(),
        spec.channels.to_string().bright_blue()
    );

    eprintln!("{message}\n");

    for effect_spec in effect_chain.iter() {
        let effect = effect_map.get(&effect_spec.name).unwrap();
        let bar = ProgressBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(100));
        bar.set_style(
            ProgressStyle::with_template(format!("Applying effect '{}' {{msg}}{{spinner}}", effect_spec.name).as_str())
            .unwrap()
        );
        
        let result = effect.apply_effect(&mut buffer, &effect_spec.arguments, &args.tail);

        let message;
        match result {
            Ok(m) => { message = m },
            Err(message) => {
                bar.finish_with_message(format!("{}", "failed".red()));
                error(message, ErrorKind::Io);
                return;
            }
        }
        match audio_utils::sanitize_buffer(&mut buffer) {
            Ok(_) => {
                if message.is_some() {
                    bar.finish_with_message(format!("... {} {}", "done".green(), format!("({})", message.unwrap()).yellow()));
                } else {
                    bar.finish_with_message(format!("... {}", "done".green()));
                }

            },
            Err(message) => {
                bar.finish_with_message(format!("... {}","failed".red()));
                error(message, ErrorKind::ValueValidation);
                return;
            }
        }
    }
    
    eprintln!("\n");
    
    let path;
    if args.overwrite {
        path = args.file_path;
    } else {
        path = args.output.unwrap();
    }

    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));
    bar.set_style(ProgressStyle::with_template(format!("Writing to {:#?} {{msg}} {{spinner}}", path).as_str()).unwrap());

    match encoder::encode_file(&buffer, path) {
        Ok(count) => {
            bar.finish_with_message(format!("... {}", "done".green()));
            if count > 0 {
                eprintln!("   Clipping: {} samples. Consider normalizing the audio or decreasing the gain.", count.to_string().yellow())
            }
        },
        Err(e) => {
            bar.finish_with_message(format!("... {}","failed".red()));
            error(e.to_string(), ErrorKind::ValueValidation);
            return;
        }
    }
    eprintln!("   Output duration: {:.2}s", get_buffer_duration(&buffer));

    eprintln!("Total processing time: {:.2?}", time.elapsed());
}
