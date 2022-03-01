use crate::hydrus_serializable::dictionary::HydrusDictionary;
use crate::hydrus_serializable::wrapper::HydrusSerWrapper;
use crate::Result;
use crate::{fix, Endpoint, FromJson, GetEndpoint};
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
    server_message: String,
    update_period: u64,
    nullification_period: u64,
    tag_filter: Value,
}

impl FromJson for OptionsResponse {
    fn from_json(value: serde_json::Value) -> Result<Self> {
        let response = serde_json::from_value::<HydrusSerWrapper<HydrusDictionary>>(value)?;
        let options_value = fix!(response.inner.get_one(&"service_options".into()));
        let options_value =
            serde_json::from_value::<HydrusSerWrapper<HydrusDictionary>>(options_value.clone())?
                .inner;

        let server_message =
            fix!(fix!(options_value.get_one(&"server_message".into())).as_str()).to_string();
        let update_period = fix!(fix!(options_value.get_one(&"update_period".into())).as_u64());
        let nullification_period =
            fix!(fix!(options_value.get_one(&"nullification_period".into())).as_u64());
        let tag_filter = fix!(options_value.get_one(&"tag_filter".into())).clone();

        Ok(Self {
            server_message,
            update_period,
            nullification_period,
            tag_filter,
        })
    }
}
