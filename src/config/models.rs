use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct IconThreshold {
    pub icon: char,
    pub level: f64,
}
