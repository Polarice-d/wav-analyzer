use std::{i8, i16, i32, path::PathBuf};

use hound::{WavWriter, SampleFormat};
use crate::types::AudioBuffer;

pub fn encode_file(buffer:AudioBuffer, filename:PathBuf) {
    let mut count: i64 = 0;
    let mut writer = WavWriter::create(filename, buffer.original_spec).unwrap();
    for sample in buffer.normalized_samples.iter() {
        if sample.abs() > 1.0  {
            count += 1;
        }
    }

    match buffer.original_spec.bits_per_sample {
       16 => {
        let amplitude = i16::MAX as f32;
        for sample in buffer.normalized_samples {
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
       },
       32 => {
        if buffer.original_spec.sample_format == SampleFormat::Int {
            let amplitude = i32::MAX as f32;
            for sample in buffer.normalized_samples {
                writer.write_sample((sample * amplitude) as i32).unwrap()
            }
        } else {
            for sample in buffer.normalized_samples {
                writer.write_sample(sample).unwrap()
            }
        }
       }
       _ => {
            panic!("cannot encode unsupported format")
       }
    }

    writer.finalize().unwrap();

    if count > 0 {
        println!("Warning, audio will clip! Counted {} samples that exceed 0 dB", count);
    }
}

