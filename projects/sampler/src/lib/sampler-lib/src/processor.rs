use crate::types::Measurement;
use crate::utils::sample;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SamplingProcessor {
    measurements: Vec<Measurement>,
}

// module for wasm bindings
#[wasm_bindgen]
impl SamplingProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SamplingProcessor {
        SamplingProcessor {
            measurements: Vec::new(),
        }
    }

    pub fn add_measurement(&mut self, timestamp: String, measurement_type: String, value: f64) {
        self.measurements.push(Measurement {
            timestamp,
            measurement_type,
            value,
        });
    }

    pub fn process_measurements(&self) -> String {
        let results = sample(&self.measurements);
        serde_json::to_string(&results).unwrap_or_default()
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
