use std::{collections::HashMap};

use crate::types::{EffectSpec};

fn parse_effect_spec(input: &str) -> Result<EffectSpec, String> {
    let buffer: Vec<&str> = input.split(":").collect();
    let mut arguments: HashMap<String, Option<String>> = HashMap::new();

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
            1 => arguments.insert(pair[0].trim().to_lowercase(), None),
            2 => arguments.insert(pair[0].trim().to_lowercase(), Some(pair[1].trim().to_lowercase())),
            _ => return Err(format!("malformed argument '{arg}' for effect '{effect_name}'"))
        };
    }

    return Ok(
        EffectSpec {
            effect_name,
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