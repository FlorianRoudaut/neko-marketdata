use serde::{Deserialize, Serialize};
use neko_tech_persistence::Key;

pub const METRIC: &str = "Metric";

#[derive(Serialize, Deserialize, Default)]
pub struct Metric {
    pub key: Key,
    pub rank: u8,
}

mod persisted_impl {
    use prost::Message;
    use neko_tech_persistence::{Key, PersistenceError, Persisted};
    use crate::proto::{MetricList, metric_to_proto, proto_to_metric};
    use super::{Metric, METRIC};

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
}
