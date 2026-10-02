use crate::market_data::MarketData;
use crate::time_series::{Step, TimeSeries};

pub fn to_log_return_series(name: &str, step: Step, data: &[&MarketData]) -> TimeSeries {
    let len = data.len();
    let mut log_returns = vec![0.0; len];
    for i in 1..data.len() {
        log_returns[i] = data[i].value.ln() - data[i - 1].value.ln();
    }
    TimeSeries { name: name.to_string(), step, values: log_returns }
}
