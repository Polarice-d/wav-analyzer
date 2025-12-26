use std::{collections::HashMap, num::ParseFloatError};

use crate::types::{EffectSpec};

fn parse_effect_spec(input: &str) -> Result<EffectSpec, String> {
    let buffer: Vec<&str> = input.split(":").collect();
    let mut arguments: HashMap<String, f32> = HashMap::new();

    let effect_name = buffer[0].trim().to_lowercase();
    if effect_name.is_empty() {
        return Err(format!("empty effect name"));
    }

    for arg in buffer.iter().skip(1) {
        if arg.is_empty() {
            return Err(format!("malformed arguments for effect '{effect_name}'"));
        }

        let pair: Vec<&str> = arg.split("=").collect();
        match pair.len() {
            2 => arguments.insert(
                pair[0].trim().to_lowercase(),
                pair[1].trim().to_lowercase().parse().map_err(|e: ParseFloatError| e.to_string())?),
            _ => return Err(format!("Malformed argument '{arg}' for effect '{effect_name}'"))
        };
    }

    return Ok(
        EffectSpec {
            name: effect_name,
            arguments: arguments
        }
    );
}

pub fn parse_effects(input: &Vec<String>) -> Result<Vec<EffectSpec>, String> {
    let mut result: Vec<EffectSpec> = Vec::new();
    
    for effect in input.iter() {
       let spec = parse_effect_spec(effect)?;
       result.push(spec); 
    };

    return Ok(result);
}

