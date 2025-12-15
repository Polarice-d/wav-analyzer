use hound::WavSpec;
use std::collections::HashMap;

pub struct AudioBuffer {
    pub original_spec: WavSpec,
    pub normalized_samples: Vec<f32>
}

pub struct EffectSpec {
    pub effect_name: String,
    pub arguments: HashMap<String, Option<String>>
}