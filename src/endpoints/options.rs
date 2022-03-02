use crate::hydrus_serializable::dictionary::HydrusDictionary;
use crate::hydrus_serializable::wrapper::HydrusSerWrapper;
use crate::Result;
use crate::{Endpoint, FromJson, GetEndpoint};
use serde_json::Value;

pub struct Options;

impl Endpoint for Options {
    fn path() -> &'static str {
        "options"
    }
}

impl GetEndpoint for Options {
    type Response = OptionsResponse;
}

#[derive(Clone, Debug)]
pub struct OptionsResponse {
    pub server_message: String,
    pub update_period: u64,
    pub nullification_period: u64,
    pub tag_filter: Value,
}

impl FromJson for OptionsResponse {
    fn from_json(value: serde_json::Value) -> Result<Self> {
        let mut response = HydrusDictionary::from_json(value)?;
        let mut service_options = response
            .take_by_str::<HydrusSerWrapper<HydrusDictionary>>("service_options")?
            .inner;

        let server_message = service_options.take_by_str::<String>("server_message")?;
        let update_period = service_options.take_by_str::<u64>("update_period")?;
        let nullification_period = service_options.take_by_str::<u64>("nullification_period")?;
        let tag_filter = service_options.take_by_str::<Value>(&"tag_filter")?;

        Ok(Self {
            server_message,
            update_period,
            nullification_period,
            tag_filter,
        })
    }
}
