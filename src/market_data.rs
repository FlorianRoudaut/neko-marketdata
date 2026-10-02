use serde::{Deserialize, Serialize};
use neko_tech_persistence::Key;
use uuid::Uuid;

pub const MARKET_DATA: &str = "MarketData";

#[derive(Serialize, Deserialize, Default)]
pub struct MarketData {
    pub key: Key,
    pub security_id: Uuid,
    pub source: u8,
    pub metric_id: u8,
    pub value: f64,
    pub timestamp: u32,
}

mod persisted_impl {
    use prost::Message;
    use neko_tech_persistence::{Key, PersistenceError, Persisted};
    use crate::proto::{MarketDataList, market_data_to_proto, proto_to_market_data};
    use super::{MarketData, MARKET_DATA};

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
