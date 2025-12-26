use std::collections::HashMap;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use crate::types::{AudioBuffer, AudioEffect};

pub struct Delay;

const COMMAND_NAME: &str = "delay";
const MIX_NAME: &str = "mix";
const FEEDBACK_NAME: &str = "fb";
const TIME_NAME: &str = "time";
const MIN_DELAY_ENERGY: f64 = 0.0001; // This number is equivalent to -80 dB in energy

impl AudioEffect for Delay {

    fn get_name(&self) -> String { COMMAND_NAME.to_string() }

    fn validate_arguments(&self, arguments: &HashMap<String, f32>, tail_length: &Option<f32>) -> Result<(), String> {
        let mix = arguments.get(MIX_NAME).ok_or_else(|| format!("Missing delay argument '{MIX_NAME}' (add '{MIX_NAME}=x' to delay:)"))?;
        let feedback = arguments.get(FEEDBACK_NAME).ok_or_else(|| format!("Missing delay argument '{FEEDBACK_NAME}' (add '{FEEDBACK_NAME}=x' to 'delay:')"))?;
        let time = arguments.get(TIME_NAME).ok_or_else(|| format!("Missing delay argument '{TIME_NAME}' (add '{TIME_NAME}=x' to 'delay:')"))?;

        if *feedback >= 1.0 && tail_length.is_none() {
            return Err("Tail length (--tail, -t) is required for delay feedback >= 1 to avoid infinite feedback cycles".to_string());
        }

        if *time < 1.0 {
            return Err("Delay time must be >= 1".to_string());
        }

        if *mix < 0.0 {
            return Err("Delay mix must be > 0".to_string());
        }

        if *feedback < 0.0 {
            return Err("Delay feedback must be > 0".to_string());
        }

        if *feedback < 1.0 && *feedback > 0.9 && tail_length.is_none() {
            println!("Warning, delay feedback level ~1 was requested! Processing may take a while and your file may be quite large");
        }



        Ok(())
    }

    fn apply_effect(&self, audio_buffer: &mut AudioBuffer, arguments: &HashMap<String,f32>, tail_length: &Option<f32>) -> Result<(), String> {
        let mix = arguments.get(MIX_NAME).unwrap();
        let feedback = arguments.get(FEEDBACK_NAME).unwrap();
        let time = arguments.get(TIME_NAME).unwrap();

        let sample_rate = audio_buffer.spec.sample_rate as i32;
        let ringbuffer_size = ((time / 1000.0) * sample_rate as f32 * audio_buffer.spec.channels as f32) as usize;
        let mut buffer: AllocRingBuffer<f32> = AllocRingBuffer::new(ringbuffer_size);
        
        buffer.enqueue(0.0);
        
        for sample in audio_buffer.samples.iter_mut() {
            let delayed = *buffer.front().unwrap();
            buffer.enqueue(*sample + delayed * feedback);
            *sample += delayed * mix;
        }
    
        let mut square_sum: f64 = buffer.iter().map(|val| (*val) as f64 * (*val) as f64 ).sum::<f64>();
        let normalizing_factor: f64 = 1.0/(ringbuffer_size as f64).sqrt();
    
        if tail_length.is_some() {
            let tail_samples = audio_buffer.spec.channels as i32 * tail_length.unwrap() as i32 * sample_rate;
            for _ in 0 .. tail_samples {
                buffer.enqueue(buffer.front().unwrap() * feedback);
                audio_buffer.samples.push((buffer.front().unwrap() * mix).clamp(-1.0, 1.0));
            }
        } else {
            let mut counter = 0;
            let mut prev_energy = f64::INFINITY;
            while normalizing_factor * square_sum.sqrt() > MIN_DELAY_ENERGY { 
                let energy = normalizing_factor * square_sum.sqrt();

                if energy < prev_energy && counter > 0 {
                    counter -= 1;
                } else if energy >= prev_energy {
                    counter += 1;
                }

                println!("{}", counter);

                if counter >= ringbuffer_size { 
                    break; // With feedback values close to 1, energy may asymptote just above the threshold due to floating point shenanigans, thus creating an infinite loop 
                }
                prev_energy = energy;

                let front_val = *buffer.front().unwrap();
                square_sum -= (front_val * front_val) as f64;
                buffer.enqueue(front_val * feedback);

                let back_val = buffer.back().unwrap();
                square_sum += (back_val * back_val) as f64;

                audio_buffer.samples.push((buffer.front().unwrap() * mix).clamp(-1.0, 1.0));
            }
        }
    
        if audio_buffer.samples.len() % 2 != 0 {
            audio_buffer.samples.push(0.0);
        }

        Ok(())
    } 
}