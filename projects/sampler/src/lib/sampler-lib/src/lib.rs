// lib.rs
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Timelike, Duration};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Measurement {
    timestamp: String,
    measurement_type: String,
    value: f64,
}

#[wasm_bindgen]
pub struct SamplingProcessor {
    measurements: Vec<Measurement>,
}

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
        let mut type_grouped: HashMap<String, Vec<&Measurement>> = HashMap::new();
        
        // Group measurements by type
        for measurement in &self.measurements {
            type_grouped
                .entry(measurement.measurement_type.clone())
                .or_default()
                .push(measurement);
        }
        
        let mut sampled_measurements: Vec<Measurement> = Vec::new();
        
        // Process each measurement type
        for (measurement_type, measurements) in type_grouped {
            let mut interval_grouped: HashMap<DateTime<chrono::Utc>, Vec<&Measurement>> = HashMap::new();
            
            // Parse and group measurements by 5-minute intervals
            for measurement in measurements {
                if let Ok(timestamp) = DateTime::parse_from_rfc3339(&measurement.timestamp) {
                    let utc_timestamp = timestamp.with_timezone(&chrono::Utc);
                    let interval = get_interval_start(utc_timestamp);
                    interval_grouped.entry(interval).or_default().push(measurement);
                }
            }
            
            // Get the latest measurement for each interval
            for (interval, interval_measurements) in interval_grouped {
                if let Some(latest) = get_latest_measurement(interval_measurements) {
                    sampled_measurements.push(latest.clone());
                }
            }
        }
        
        // Sort by timestamp and measurement type
        sampled_measurements.sort_by(|a, b| {
            let timestamp_cmp = a.timestamp.cmp(&b.timestamp);
            if timestamp_cmp == std::cmp::Ordering::Equal {
                a.measurement_type.cmp(&b.measurement_type)
            } else {
                timestamp_cmp
            }
        });
        
        // Convert to JSON string
        serde_json::to_string(&sampled_measurements).unwrap_or_default()
    }
}

fn get_interval_start(timestamp: DateTime<chrono::Utc>) -> DateTime<chrono::Utc> {
    let minutes = timestamp.minute();
    let interval = (minutes / 5) * 5;
    timestamp
        .with_minute(interval).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap()
}

fn get_latest_measurement<'a>(measurements: Vec<&'a Measurement>) -> Option<&'a Measurement> {
    measurements.into_iter().max_by_key(|m| m.timestamp.clone())
}

// Required for wasm-pack
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
