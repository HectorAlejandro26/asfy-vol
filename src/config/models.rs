use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IconThreshold {
    pub icon: char,
    pub level: f64,
}
