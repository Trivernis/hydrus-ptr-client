use crate::hydrus_serializable::dictionary::HydrusDictionary;
use crate::hydrus_serializable::metadata::HydrusMetadata;
use crate::hydrus_serializable::wrapper::HydrusSerWrapper;
use crate::{Endpoint, FromJson, GetEndpoint};
use serde_json::Value;

pub struct MetadataEndpoint;

impl Endpoint for MetadataEndpoint {
    fn path() -> &'static str {
        "metadata"
    }
}

impl GetEndpoint for MetadataEndpoint {
    type Response = MetadataResponse;
}

#[derive(Clone, Debug)]
pub struct MetadataResponse(pub HydrusMetadata);

impl FromJson for MetadataResponse {
    fn from_json(value: Value) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut dict = HydrusDictionary::from_json(value)?;
        let metadata = dict
            .take_by_str::<HydrusSerWrapper<HydrusMetadata>>("metadata_slice")?
            .inner;

        Ok(MetadataResponse(metadata))
    }
}

impl MetadataResponse {
    pub fn update_hashes(&self) -> Vec<&String> {
        self.0
            .entries
            .iter()
            .flat_map(|e| e.update_hashes.iter().collect::<Vec<&String>>())
            .collect()
    }
}
