use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Step {
    Minutes5,
    Hour1,
    Day1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimeSeries {
    pub name: String,
    pub step: Step,
    pub values: Vec<f64>,
}
