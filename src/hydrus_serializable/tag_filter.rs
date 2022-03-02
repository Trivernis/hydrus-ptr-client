use crate::constants::HYDRUS_TYPE_TAG_FILTER;
use crate::hydrus_serializable::HydrusSerializable;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct HydrusTagFilter(pub Value);

impl HydrusSerializable for HydrusTagFilter {
    fn type_id() -> u64 {
        HYDRUS_TYPE_TAG_FILTER
    }
}
