use crate::constants::{
    CONTENT_TYPE_MAPPINGS, CONTENT_TYPE_TAG_PARENTS, CONTENT_TYPE_TAG_SIBLINGS,
    HYDRUS_TYPE_CONTENT_UPDATE,
};
use crate::hydrus_serializable::HydrusSerializable;
use crate::{Error, Result};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct HydrusContentUpdate(Vec<ContentUpdateEntries>);

impl HydrusSerializable for HydrusContentUpdate {
    fn type_id() -> u64 {
        HYDRUS_TYPE_CONTENT_UPDATE
    }
}

impl HydrusContentUpdate {
    pub fn take<U: ContentUpdateTrait>(
        &mut self,
    ) -> Result<Option<Vec<ContentUpdatesAndAction<U>>>> {
        if let Some(index) = self.0.iter().position(|e| e.content_type == U::type_id()) {
            let entry = self.0.swap_remove(index);

            serde_json::from_value(entry.entries).map_err(Error::from)
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContentUpdateEntries {
    pub content_type: u64,
    pub entries: Value,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContentUpdatesAndAction<T> {
    pub action: u64,
    pub updates: Vec<T>,
}

pub trait ContentUpdateTrait: DeserializeOwned {
    fn type_id() -> u64;
}

#[derive(Clone, Debug, Deserialize)]
pub struct MappingsUpdateEntry {
    pub tag_id: u64,
    pub hash_ids: Vec<u64>,
}

impl ContentUpdateTrait for MappingsUpdateEntry {
    fn type_id() -> u64 {
        CONTENT_TYPE_MAPPINGS
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TagParentsUpdateEntry {
    pub child_id: u64,
    pub parent_id: u64,
}

impl ContentUpdateTrait for TagParentsUpdateEntry {
    fn type_id() -> u64 {
        CONTENT_TYPE_TAG_PARENTS
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TagSiblingsUpdateEntry {
    pub tag_id: u64,
    pub sibling_id: u64,
}

impl ContentUpdateTrait for TagSiblingsUpdateEntry {
    fn type_id() -> u64 {
        CONTENT_TYPE_TAG_SIBLINGS
    }
}
