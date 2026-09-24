use uuid::Uuid;
use neko_tech_persistence::Key;
use crate::{Metric, Source, MarketData};

mod generated {
    include!(concat!(env!("OUT_DIR"), "/neko.rs"));
}

pub use generated::MetricList;
pub use generated::SourceList;
pub use generated::MarketDataList;

pub fn metric_to_proto(m: &Metric) -> generated::Metric {
    generated::Metric {
        key: Some(key_to_proto(&m.key)),
        rank: m.rank as u32,
    }
}

pub fn proto_to_metric(p: generated::Metric) -> Metric {
    Metric {
        key: proto_to_key(p.key.unwrap_or_default()),
        rank: p.rank as u8,
    }
}

pub fn source_to_proto(s: &Source) -> generated::Source {
    generated::Source {
        key: Some(key_to_proto(&s.key)),
        rank: s.rank as u32,
    }
}

pub fn proto_to_source(p: generated::Source) -> Source {
    Source {
        key: proto_to_key(p.key.unwrap_or_default()),
        rank: p.rank as u8,
    }
}

pub fn market_data_to_proto(m: &MarketData) -> generated::MarketData {
    generated::MarketData {
        key: Some(key_to_proto(&m.key)),
        source: m.source as u32,
        metric_id: m.metric_id as u32,
        security_id: m.security_id.to_string(),
        value: m.value,
        timestamp: m.timestamp,
    }
}

pub fn proto_to_market_data(p: generated::MarketData) -> MarketData {
    MarketData {
        key: proto_to_key(p.key.unwrap_or_default()),
        source: p.source as u8,
        metric_id: p.metric_id as u8,
        security_id: p.security_id.parse().unwrap_or_else(|_| Uuid::new_v4()),
        value: p.value,
        timestamp: p.timestamp,
    }
}

fn key_to_proto(k: &Key) -> generated::Key {
    generated::Key {
        id: k.id.to_string(),
        version: k.version,
        unique_name: k.unique_name.clone(),
    }
}

fn proto_to_key(k: generated::Key) -> Key {
    Key {
        id: k.id.parse().unwrap_or_else(|_| Uuid::new_v4()),
        version: k.version,
        unique_name: k.unique_name,
    }
}
