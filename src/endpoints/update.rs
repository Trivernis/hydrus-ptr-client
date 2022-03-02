use crate::hydrus_serializable::definitions_update::{
    HashDefinition, HydrusDefinitionsUpdate, TagDefinition,
};
use crate::hydrus_serializable::wrapper::GenericHydrusSerWrapper;
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
}

#[derive(Clone, Debug)]
pub struct DefinitionsUpdateResponse {
    pub hashes: HashMap<u64, String>,
    pub tags: HashMap<u64, String>,
}

impl FromJson for UpdateResponse {
    fn from_json(value: Value) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let wrapper = serde_json::from_value::<GenericHydrusSerWrapper>(value)?;
        match wrapper.type_id {
            36 => {
                let definitions_update = DefinitionsUpdateResponse::from_wrapper(wrapper)?;

                Ok(Self::Definitions(definitions_update))
            }
            _ => Err(Error::Malformed),
        }
    }
}

impl DefinitionsUpdateResponse {
    fn from_wrapper(wrapper: GenericHydrusSerWrapper) -> Result<Self> {
        let mut definitions_update = wrapper.into_inner::<HydrusDefinitionsUpdate>()?;

        let hashes = definitions_update
            .take::<HashDefinition>()?
            .map(|h| h.into_iter().map(|h| (h.id, h.hash)).collect())
            .unwrap_or_else(|| HashMap::new());

        let tags = definitions_update
            .take::<TagDefinition>()?
            .map(|t| t.into_iter().map(|t| (t.id, t.tag)).collect())
            .unwrap_or_else(|| HashMap::new());

        Ok(Self { hashes, tags })
    }
}
