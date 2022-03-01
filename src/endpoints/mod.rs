mod options;

use crate::Result;
use std::fmt::Debug;

pub use options::*;

#[macro_export]
macro_rules! fix {
    ($opt:expr) => {
        $opt.ok_or_else(|| crate::Error::Malformed)?
    };
}

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
