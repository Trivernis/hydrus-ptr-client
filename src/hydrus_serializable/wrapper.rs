use crate::hydrus_serializable::{ConstNumberTrait, HydrusSerializable, SerializableId};
use crate::{Error, Result};
use serde::Deserialize;
use serde_json::Value;

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

/// A generic hydrus serializable wrapper that allows one
/// to retrieve the type id and act on that
#[derive(Clone, Debug, Deserialize)]
pub struct GenericHydrusSerWrapper {
    pub type_id: u64,
    #[allow(unused)]
    pub version: SerializableId<VersionOne>,
    pub inner: Value,
}

impl GenericHydrusSerWrapper {
    /// Converts the inner value into the target deserializable format
    pub fn into_inner<T: HydrusSerializable>(self) -> Result<T> {
        if self.type_id == T::type_id() {
            serde_json::from_value(self.inner).map_err(Error::from)
        } else {
            Err(Error::Malformed)
        }
    }
}
