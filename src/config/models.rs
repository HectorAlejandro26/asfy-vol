use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct IconThreshold {
    pub icon: String,
    pub level: f64,
}
