mod client_core;
mod update_stream;

pub use crate::endpoints::*;
use crate::{ClientBuilder, Result};
pub use client_core::*;
use std::fmt::Debug;
pub use update_stream::*;

#[derive(Clone)]
pub struct Client {
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) access_key: String,
}

impl Client {
    /// Creates a new client builder
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Creates a new PTR Client
    pub fn new<S1: ToString, S2: ToString>(endpoint: S1, access_key: S2) -> Self {
        Self {
            base_url: endpoint.to_string(),
            client: reqwest::Client::new(),
            access_key: access_key.to_string(),
        }
    }

    /// Returns the options of the PTR
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_options(&self) -> Result<OptionsResponse> {
        self.get::<OptionsEndpoint, ()>(&()).await
    }

    /// Returns information about all available updates since the given ID
    /// and when the next check for updates should be made
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_metadata(&self, since: u64) -> Result<MetadataResponse> {
        self.get::<MetadataEndpoint, _>(&[("since", since)]).await
    }

    /// Returns the parsed update file identified by the given hash.
    /// The hash can be retrieved by fetching the metadata with [Client::metadata]
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_update<S: AsRef<str> + Debug>(
        &self,
        update_hash: S,
    ) -> Result<UpdateResponse> {
        self.get::<UpdateEndpoint, _>(&[("update_hash", update_hash.as_ref())])
            .await
    }
}
