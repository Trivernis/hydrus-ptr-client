use crate::hydrus_serializable::content_update::{
    ContentUpdatesAndAction, HydrusContentUpdate, MappingsUpdateEntry, TagParentsUpdateEntry,
    TagSiblingsUpdateEntry,
};
use crate::hydrus_serializable::definitions_update::{
    HashDefinition, HydrusDefinitionsUpdate, TagDefinition,
};
use crate::hydrus_serializable::wrapper::GenericHydrusSerWrapper;
use crate::Error::Malformed;
use crate::Result;
use crate::{Endpoint, Error, FromJson, GetEndpoint};
use serde_json::Value;
use std::collections::HashMap;

pub struct UpdateEndpoint;

impl Endpoint for UpdateEndpoint {
    fn path() -> &'static str {
        "update"
    }
}

impl GetEndpoint for UpdateEndpoint {
    type Response = UpdateResponse;
}

#[derive(Clone, Debug)]
pub enum UpdateResponse {
    Definitions(DefinitionsUpdateResponse),
    Content(ContentUpdateResponse),
}

impl FromJson for UpdateResponse {
    fn from_json(value: Value) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let wrapper = serde_json::from_value::<GenericHydrusSerWrapper>(value)?;
        match wrapper.type_id {
            34 => {
                let content_update = ContentUpdateResponse::from_wrapper(wrapper)?;

                Ok(Self::Content(content_update))
            }
            36 => {
                let definitions_update = DefinitionsUpdateResponse::from_wrapper(wrapper)?;

                Ok(Self::Definitions(definitions_update))
            }
            _ => Err(Error::Malformed),
        }
    }
}

trait FromWrapper {
    fn from_wrapper(wrapper: GenericHydrusSerWrapper) -> Result<Self>
    where
        Self: Sized;
}

#[derive(Clone, Debug)]
pub struct DefinitionsUpdateResponse {
    pub hashes: HashMap<u64, String>,
    pub tags: HashMap<u64, String>,
}

impl FromWrapper for DefinitionsUpdateResponse {
    fn from_wrapper(wrapper: GenericHydrusSerWrapper) -> Result<Self> {
        let mut definitions_update = wrapper.into_inner::<HydrusDefinitionsUpdate>()?;

        let hashes = definitions_update
            .take::<HashDefinition>()?
            .map(|h| h.into_iter().map(|h| (h.id, h.hash)).collect())
            .unwrap_or_default();

        let tags = definitions_update
            .take::<TagDefinition>()?
            .map(|t| t.into_iter().map(|t| (t.id, t.tag)).collect())
            .unwrap_or_default();

        Ok(Self { hashes, tags })
    }
}

#[derive(Clone, Debug)]
pub struct ContentUpdateResponse {
    pub mappings: HashMap<ContentUpdateAction, HashMap<u64, Vec<u64>>>,
    pub tag_parents: HashMap<ContentUpdateAction, HashMap<u64, u64>>,
    pub tag_siblings: HashMap<ContentUpdateAction, HashMap<u64, u64>>,
}

impl FromWrapper for ContentUpdateResponse {
    fn from_wrapper(wrapper: GenericHydrusSerWrapper) -> Result<Self> {
        let mut content_update = wrapper.into_inner::<HydrusContentUpdate>()?;

        let mappings = content_update
            .take::<MappingsUpdateEntry>()?
            .map(Self::map_mappings_update)
            .unwrap_or_default();

        let tag_parents = content_update
            .take::<TagParentsUpdateEntry>()?
            .map(Self::map_tag_parents_update)
            .unwrap_or_default();

        let tag_siblings = content_update
            .take::<TagSiblingsUpdateEntry>()?
            .map(Self::map_tag_siblings_update)
            .unwrap_or_default();

        Ok(Self {
            mappings,
            tag_parents,
            tag_siblings,
        })
    }
}

impl ContentUpdateResponse {
    fn map_mappings_update(
        update: Vec<ContentUpdatesAndAction<MappingsUpdateEntry>>,
    ) -> HashMap<ContentUpdateAction, HashMap<u64, Vec<u64>>> {
        update
            .into_iter()
            .filter_map(Self::map_update_and_action)
            .map(|(action, entries)| {
                (
                    action,
                    entries
                        .into_iter()
                        .map(|e| (e.tag_id, e.hash_ids))
                        .collect::<HashMap<u64, Vec<u64>>>(),
                )
            })
            .collect()
    }

    fn map_tag_parents_update(
        update: Vec<ContentUpdatesAndAction<TagParentsUpdateEntry>>,
    ) -> HashMap<ContentUpdateAction, HashMap<u64, u64>> {
        update
            .into_iter()
            .filter_map(Self::map_update_and_action::<TagParentsUpdateEntry>)
            .map(|(action, entries)| {
                (
                    action,
                    entries
                        .into_iter()
                        .map(|e| (e.child_id, e.parent_id))
                        .collect::<HashMap<u64, u64>>(),
                )
            })
            .collect()
    }

    fn map_tag_siblings_update(
        update: Vec<ContentUpdatesAndAction<TagSiblingsUpdateEntry>>,
    ) -> HashMap<ContentUpdateAction, HashMap<u64, u64>> {
        update
            .into_iter()
            .filter_map(Self::map_update_and_action::<TagSiblingsUpdateEntry>)
            .map(|(a, entries)| {
                (
                    a,
                    entries
                        .into_iter()
                        .map(|e| (e.tag_id, e.sibling_id))
                        .collect::<HashMap<u64, u64>>(),
                )
            })
            .collect()
    }

    fn map_update_and_action<T>(
        entry: ContentUpdatesAndAction<T>,
    ) -> Option<(ContentUpdateAction, Vec<T>)> {
        Some((
            ContentUpdateAction::from_number(entry.action).ok()?,
            entry.updates,
        ))
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ContentUpdateAction {
    Add = 0,
    Delete = 1,
}

impl ContentUpdateAction {
    pub fn from_number(num: u64) -> Result<Self> {
        match num {
            0 => Ok(Self::Add),
            1 => Ok(Self::Delete),
            _ => Err(Malformed),
        }
    }
}
