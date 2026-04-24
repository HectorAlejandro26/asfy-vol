use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IconThreshold {
    pub icon: char,
    pub level: f64,
}
