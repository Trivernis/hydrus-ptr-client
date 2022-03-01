use crate::hydrus_serializable::{HydrusSerializable, SerializableId};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(bound = "")]
pub struct HydrusSerWrapper<T: HydrusSerializable> {
    pub type_id: SerializableId<T>,
    pub version: u8,
    pub inner: T,
}
