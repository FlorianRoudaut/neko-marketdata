use serde::{Deserialize, Serialize};
use neko_tech_persistence::Key;

pub const SOURCE: &str = "Source";

#[derive(Serialize, Deserialize, Default)]
pub struct Source {
    pub key: Key,
    pub rank: u8,
}

mod persisted_impl {
    use prost::Message;
    use neko_tech_persistence::{Key, PersistenceError, Persisted};
    use crate::proto::{SourceList, source_to_proto, proto_to_source};
    use super::{Source, SOURCE};

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
}
