use crate::hydrus_serializable::{ConstNumberTrait, HydrusSerializable, SerializableId};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct VersionOne;

impl ConstNumberTrait for VersionOne {
    fn value() -> u64 {
        1
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(bound = "")]
pub struct HydrusSerWrapper<T: HydrusSerializable> {
    #[allow(unused)]
    pub type_id: SerializableId<T>,
    #[allow(unused)]
    pub version: SerializableId<VersionOne>,
    pub inner: T,
}
