use core::f64;
use std::{collections::HashMap};
use crate::{types::{AudioBuffer, AudioEffect}};

pub struct EqBand;

const COMMAND_NAME: &str = "eqband";
const DB_NAME: &str = "db";
const FREQ_NAME :&str = "freq";
const BW_NAME: &str = "q";


impl AudioEffect for EqBand {
    fn get_name(&self) -> String {
        COMMAND_NAME.to_string()
    }

    fn validate_arguments(&self, arguments: &HashMap<String, f32>, tail_length: &Option<f32>) -> Result<(), String> {
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, arguments: &HashMap<String, f32>, _tail_length: &Option<f32>) -> Result<(), String> {
        let fs = buffer.spec.sample_rate as f64;
        let f0 = *arguments.get(FREQ_NAME).unwrap() as f64;
        let db_gain = *arguments.get(DB_NAME).unwrap() as f64;
        let q = *arguments.get(BW_NAME).unwrap() as f64;
        let a= 10.0_f64.powf(db_gain/40.0) as f64;
        let w0 = (2.0 * f64::consts::PI * (f0/fs)) as f64;
        let sinw0 = w0.sin();
        let cosw0 = w0.cos();
        let alpha = sinw0 /(2.0 * q);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cosw0;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha/a;
        let a1 = -2.0 * cosw0;
        let a2 = 1.0 - alpha/a;

        let mut x1 = 0.0;  // x[n-1]
        let mut x2 = 0.0;  // x[n-2]
        let mut y1 = 0.0;  // y[n-1] 
        let mut y2 = 0.0;  // y[n-2]

        for sample in buffer.samples.iter_mut() {
            let x0 = *sample as f64;
            let y0 = (b0/a0) * x0 + (b1/a0) * x1 + (b2/a0) * x2 - (a1/a0) * y1 - (a2/a0) * y2;

            *sample = y0 as f32;  

            x2 = x1;
            x1 = x0;
            y2 = y1;
            y1 = y0;
        }

        Ok(())

    }

}
