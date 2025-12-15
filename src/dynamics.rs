use std::collections::HashMap;

use crate::types::AudioBuffer;

pub fn gain(buffer:&mut AudioBuffer, arguments: &HashMap<String, Option<String>>) -> Result<(), String> {
    let gain = match arguments.get("db") {
        Some(v) => v,
        None => return Err("required gain parameter 'db' not provided".into())
    };

    let gain = match gain {
        Some(v) => v,
        None => return Err("required gain parameter 'db' is empty".into())
    };

    let gain: f32 = match gain.parse() {
        Ok(v) => v,
        Err(_) => return Err(format!("failed to parse value {gain}"))
    };

    let factor = 10.0_f32.powf(gain / 20.0);
    // cant do this with iterators
    for buf in buffer.normalized_samples.iter_mut() {
        *buf *= factor; 
    };

    Ok(())
}

pub fn distortion(buffer: &mut AudioBuffer, arguments: &HashMap<String, Option<String>>) -> Result<(), String> {
    let gain = match arguments.get("db") {
        Some(v) => v,
        None => return Err("required gain parameter 'db' not provided".into())
    };

    let gain = match gain {
        Some(v) => v,
        None => return Err("required gain parameter 'db' is empty".into())
    };

    let gain: f32 = match gain.parse() {
        Ok(v) => v,
        Err(_) => return Err(format!("failed to parse value {gain}"))
    };

    let factor = 10.0_f32.powf(gain / 20.0);

    for buf in buffer.normalized_samples.iter_mut() {
        *buf = (*buf * factor).tanh();
    };

    return Ok(());
}
