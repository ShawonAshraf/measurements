// lib.rs
use wasm_bindgen::prelude::*;
use chrono::{DateTime, Timelike, Duration, Utc, NaiveDateTime, TimeZone};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Measurement {
    timestamp: String,
    measurement_type: String,
    value: f64,
}



fn get_interval_start(timestamp: DateTime<chrono::Utc>) -> DateTime<chrono::Utc> {
    let minutes = timestamp.minute();
    let interval = (minutes / 5) * 5;
    timestamp
        .with_minute(interval).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap()
}


fn str_to_utc(ts: &String) -> DateTime<chrono::Utc> {
    let naive_ts = NaiveDateTime::parse_from_str(&ts, "%Y-%m-%dT%H:%M:%S")
        .unwrap();
    Utc.from_utc_datetime(&naive_ts)

}

fn utc_to_str(utc: &DateTime<chrono::Utc>) -> String {
    let str = utc.to_string();
    let parts = str.split(" ").collect::<Vec<&str>>();
    let formatted_str = parts[0].to_string() + "T" + parts[1];

    formatted_str

}

fn round_timestamp_to_boundary(ts: &String) -> String{
    let dt = str_to_utc(ts);
    let start = get_interval_start(dt);
    let duration = Duration::minutes(5);
    let interval = start + duration;

    utc_to_str(&interval)
}


fn find_latest_in_interval(samples: &Vec<&Measurement>) -> Vec<usize> {
    let mut timestamps: Vec<DateTime<chrono::Utc>> = Vec::new();
    let mut index_map: HashMap<DateTime<chrono::Utc>, usize> = HashMap::new();

    for i in 0..samples.len() {
        let parsed = str_to_utc(&samples[i].timestamp);
        timestamps.push(parsed);
        index_map.insert(parsed, i);
    }


    timestamps.sort();


    let mut current_interval_start = get_interval_start(timestamps[0]);
    let mut current_latest = timestamps[0];
    let mut latest_in_intervals: Vec<DateTime<Utc>> = Vec::new();
    let interval_duration = Duration::minutes(5);


    let n = timestamps.len();
    for i in 0..n {
        let t = timestamps[i];
        if t <= current_interval_start + interval_duration {
            if t > current_latest {
                current_latest = t;
            }
        } else {
            latest_in_intervals.push(current_latest);
            current_interval_start = get_interval_start(t);
            current_latest = t;
        }

        // if at the loop boundary
        if i + 1 == n {
            latest_in_intervals.push(t);
        }

    }


    let mut results: Vec<usize> = Vec::new();
    for v in latest_in_intervals {
        let idx = index_map.get(&v).unwrap();
        results.push(*idx);
    }

    results
}

fn sample(samples: &Vec<Measurement>) -> Vec<Measurement> {
    let mut results: Vec<Measurement> = Vec::new();

    // group by type
    let mut type_grouped: HashMap<String, Vec<&Measurement>> = HashMap::new();

    for sample in samples {
        type_grouped
            .entry(sample.measurement_type.clone())
            .or_default()
            .push(sample);
    }

    let keys: Vec<String> = type_grouped.keys().cloned().collect();


    for key in keys {
        // get the samples for key
        let samples_k = type_grouped.get(&key).unwrap();

        // find the latest values in an interval
        let latest_idxs = find_latest_in_interval(samples_k);

        // collect latest timestamps
        // and create updated measurements
        for idx in latest_idxs {
            let ts = samples_k[idx].timestamp.clone();
            let rounded = round_timestamp_to_boundary(&ts);

            results.push(
                Measurement {
                    timestamp: rounded,
                    measurement_type: samples_k[idx].measurement_type.clone(),
                    value: samples_k[idx].value.clone(),
                }
            )
        }
    }


    results
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
        let results = sample(&self.measurements);
        serde_json::to_string(&results).unwrap_or_default()
    }
}



#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
