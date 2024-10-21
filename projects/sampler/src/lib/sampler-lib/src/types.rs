use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Measurement {
    pub timestamp: String,
    pub measurement_type: String,
    pub value: f64,
}
