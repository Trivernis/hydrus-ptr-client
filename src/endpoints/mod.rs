mod metadata;
mod options;

use crate::Result;
use std::fmt::Debug;

pub use metadata::*;
pub use options::*;

pub trait Endpoint {
    fn path() -> &'static str;
}

pub trait GetEndpoint: Endpoint {
    type Response: FromJson + Debug;
}

pub trait PostEndpoint: Endpoint {
    type Request;
    type Response: FromJson + Debug;
}

pub trait FromJson {
    fn from_json(value: serde_json::Value) -> Result<Self>
    where
        Self: Sized;
}
