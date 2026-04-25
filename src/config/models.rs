use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IconThreshold {
    pub icon: char,
    pub level: f64,
}
