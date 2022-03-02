use crate::constants::HYDRUS_TYPE_DEFINITIONS_UPDATE;
use crate::hydrus_serializable::HydrusSerializable;
use crate::{Error, Result};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct HydrusDefinitionsUpdate(pub Vec<DefinitionsUpdateEntries>);

impl HydrusDefinitionsUpdate {
    pub fn take<D: DefinitionsTrait>(&mut self) -> Result<Option<Vec<D>>> {
        let entry_index = self
            .0
            .iter()
            .position(|d| d.definition_id == D::definition_id());
        if let Some(idx) = entry_index {
            let entry = self.0.swap_remove(idx);

            entry.into_inner()
        } else {
            Ok(None)
        }
    }
}

impl HydrusSerializable for HydrusDefinitionsUpdate {
    fn type_id() -> u64 {
        HYDRUS_TYPE_DEFINITIONS_UPDATE
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DefinitionsUpdateEntries {
    pub definition_id: u64,
    entries: Value,
}

impl DefinitionsUpdateEntries {
    pub fn into_inner<T: DeserializeOwned>(self) -> Result<T> {
        serde_json::from_value::<T>(self.entries).map_err(Error::from)
    }
}

pub trait DefinitionsTrait: DeserializeOwned {
    fn definition_id() -> u64;
}

#[derive(Deserialize, Clone, Debug)]
pub struct HashDefinition {
    pub id: u64,
    pub hash: String,
}

impl DefinitionsTrait for HashDefinition {
    fn definition_id() -> u64 {
        0
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct TagDefinition {
    pub id: u64,
    pub tag: String,
}

impl DefinitionsTrait for TagDefinition {
    fn definition_id() -> u64 {
        1
    }
}
