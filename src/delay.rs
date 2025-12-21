use std::collections::HashMap;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use crate::types::AudioBuffer;

pub fn delay(audio_buffer:&mut AudioBuffer, arguments: &HashMap<String,Option<String>>, tail_length: Option<f32>) {
    let mix:f32 = arguments.get("mix")
    .unwrap()
    .as_ref()
    .unwrap()
    .parse()
    .expect("Invalid mix argument"); 

    let feedback :f32 = arguments.get("feedback")
    .unwrap()
    .as_ref()
    .unwrap()
    .parse()
    .expect("Invalid feedback argument");

    let time: i32 = arguments.get("time")
    .unwrap()
    .as_ref()
    .unwrap()
    .parse()
    .expect("Invalid time argument");

    if feedback >= 1.0 && tail_length.is_none() {
        panic!("tail length (--tail, -t) is required for delay feedback >= 1 to avoid infinite feedback cycles")
    }

    if feedback < 1.0 && feedback > 0.9 && tail_length.is_none() {
        println!("Warning, delay feedback level ~1 was requested. Processing may take a while and your file may be quite large")
    }

    let sample_rate = audio_buffer.original_spec.sample_rate as i32;
    let ringbuffer_size = ((time as f32 / 1000.0) * sample_rate as f32 * audio_buffer.original_spec.channels as f32) as usize;
    let mut buffer: AllocRingBuffer<f32> = AllocRingBuffer::new(ringbuffer_size);

    buffer.enqueue(0.0);

    for sample in audio_buffer.normalized_samples.iter_mut() {
        buffer.enqueue((*sample + buffer.front().unwrap()) * feedback);
        *sample += buffer.front().unwrap() * mix;
    }

    let mut square_sum: f32 = buffer.iter().map(|val| val.powf(2.0)).sum();
    let normalizing_factor: f32 = (1.0/(ringbuffer_size as f32)).sqrt();

    if tail_length.is_some() {
        let tail_samples = audio_buffer.original_spec.channels as i32 * (tail_length.unwrap() * sample_rate as f32) as i32;
        for _ in 0 .. tail_samples {
            buffer.enqueue(buffer.front().unwrap() * feedback);
            audio_buffer.normalized_samples.push(buffer.front().unwrap() * mix);
        }
    } else {
        while normalizing_factor * square_sum.sqrt() > 0.0001 { // This magic number (0.0001) is -80 dB as relative energy, AKA the amplitude of the sound
            let front_val = *buffer.front().unwrap();
            square_sum -= front_val.powf(2.0);
            buffer.enqueue(front_val * feedback);
            square_sum += buffer.back().unwrap().powf(2.0);
            audio_buffer.normalized_samples.push(buffer.front().unwrap() * mix);
        }
    }

    if audio_buffer.normalized_samples.len() % 2 != 0 {
        audio_buffer.normalized_samples.push(0.0);
    }
}