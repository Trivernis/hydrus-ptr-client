use crate::hydrus_serializable::HydrusSerializable;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct HydrusMetadata {
    pub entries: Vec<MetadataEntry>,
    pub next_update_due: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MetadataEntry {
    pub update_index: u64,
    pub update_hashes: Vec<String>,
    pub time_begin: u64,
    pub time_end: u64,
}

impl HydrusSerializable for HydrusMetadata {
    fn type_id() -> u64 {
        37
    }
}
