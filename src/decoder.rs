use crate::types::AudioBuffer;
use std::fs::read;
use std::{path::PathBuf};
use hound::WavReader;
use hound::SampleFormat::{Float,Int};

pub fn read_and_normalize_wav(path:&PathBuf) -> Result<AudioBuffer, String> {
    let mut reader = match WavReader::open(&path) {
        Ok(val) => val,
        Err(val) => {return Err(val.to_string());}
    };

    let spec = reader.spec();

    println!("Reading file {:#?}.\n{:#?}", &path, spec);
    
    /*let normalized_samples: Vec<f32> = match spec.bits_per_sample {
        
        // you have no idea how long it took me to figure out how to turn this into a vector 💀
        // NOTE: 8 bit PCM wav files typically use *unsigned* 8 bit integers

        8 => {let vector_buf:Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();
              vector_buf.into_iter().map(|i8_sample| i8_sample as f32 / i8::MAX as f32).collect()
            },
        16 => {let vector_buf:Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();
              vector_buf.into_iter().map(|i16_sample| i16_sample as f32 / i16::MAX as f32).collect()},

        32 => {match spec.sample_format {
            Int => {
              let vector_buf:Vec<i32> = reader.samples::<i32>().map(|s| s.unwrap()).collect();
              vector_buf.into_iter().map(|i32_sample| i32_sample as f32 / i32::MAX as f32).collect()
            },
            Float => {reader.samples::<f32>().map(|s| s.unwrap()).collect()}
        }},
        _ => {return Err("Unsupported format!".to_string());}
    };  */

    let normalized_samples: Vec<f32> = match spec.bits_per_sample {
        16 => reader
            .samples::<i16>()
            .map(|s| (s.unwrap() as f32) / i16::MAX as f32)
            .collect(),
        32 => match spec.sample_format {
            Int => reader
                .samples::<i32>()
                .map(|s| s.unwrap() as f32 / i32::MAX as f32)
                .collect(),
            Float => reader
                .samples::<f32>()
                .map(|s| s.unwrap())
                .collect(),
        },
        _ => return Err("Unsupported .wav format".to_string())
    };

    return Ok(AudioBuffer {
        original_spec: spec,
        normalized_samples
    });    
}