use serde::{Deserialize, Serialize};
use neko_tech_persistence::Key;
use uuid::Uuid;

mod proto;

pub const METRIC: &str = "Metric";
pub const SOURCE: &str = "Source";
pub const MARKET_DATA: &str = "MarketData";

#[derive(Serialize, Deserialize, Default)]
pub struct Metric {
    pub key: Key,
    pub rank: u8,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Source {
    pub key: Key,
    pub rank: u8,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MarketData {
    pub key: Key,
    pub security_id: Uuid,
    pub source: u8,
    pub metric_id: u8,
    pub value: f64,
    pub timestamp: u32,
}

mod persisted_impls {
    use prost::Message;
    use neko_tech_persistence::{Key, PersistenceError, Persisted};
    use crate::{Metric, Source, MarketData, METRIC, SOURCE, MARKET_DATA};
    use crate::proto::{
        MetricList, SourceList, MarketDataList,
        metric_to_proto, proto_to_metric,
        source_to_proto, proto_to_source,
        market_data_to_proto, proto_to_market_data,
    };

    impl Persisted for Metric {
        fn key(&self) -> &Key { &self.key }
        fn persisted_type() -> &'static str { METRIC }

        fn to_proto_bytes(items: &[Metric]) -> Vec<u8> {
            MetricList { metrics: items.iter().map(metric_to_proto).collect() }.encode_to_vec()
        }

        fn from_proto_bytes(bytes: &[u8]) -> Result<Vec<Metric>, PersistenceError> {
            MetricList::decode(bytes)
                .map(|l| l.metrics.into_iter().map(proto_to_metric).collect())
                .map_err(|e| PersistenceError::StorageError(e.to_string()))
        }
    }

    impl Persisted for Source {
        fn key(&self) -> &Key { &self.key }
        fn persisted_type() -> &'static str { SOURCE }

        fn to_proto_bytes(items: &[Source]) -> Vec<u8> {
            SourceList { sources: items.iter().map(source_to_proto).collect() }.encode_to_vec()
        }

        fn from_proto_bytes(bytes: &[u8]) -> Result<Vec<Source>, PersistenceError> {
            SourceList::decode(bytes)
                .map(|l| l.sources.into_iter().map(proto_to_source).collect())
                .map_err(|e| PersistenceError::StorageError(e.to_string()))
        }
    }

    impl Persisted for MarketData {
        fn key(&self) -> &Key { &self.key }
        fn persisted_type() -> &'static str { MARKET_DATA }

        fn to_proto_bytes(items: &[MarketData]) -> Vec<u8> {
            MarketDataList { market_data: items.iter().map(market_data_to_proto).collect() }.encode_to_vec()
        }

        fn from_proto_bytes(bytes: &[u8]) -> Result<Vec<MarketData>, PersistenceError> {
            MarketDataList::decode(bytes)
                .map(|l| l.market_data.into_iter().map(proto_to_market_data).collect())
                .map_err(|e| PersistenceError::StorageError(e.to_string()))
        }
    }
}
