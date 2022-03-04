use crate::constants::{DEFAULT_PTR_ADDRESS, DEFAULT_READONLY_ACCESS_KEY};
use crate::Client;
use crate::{Error, Result};
use std::time::Duration;

pub struct ClientBuilder {
    reqwest_builder: reqwest::ClientBuilder,
    endpoint: String,
    access_key: Option<String>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            reqwest_builder: reqwest::ClientBuilder::new(),
            endpoint: String::from(DEFAULT_PTR_ADDRESS),
            access_key: Some(String::from(DEFAULT_READONLY_ACCESS_KEY)),
        }
    }
}

impl ClientBuilder {
    /// Doesn't validate ssl certificates of the endpoint.
    ///
    /// # Warning
    /// Turning this on allows invalid and expired certificates which is a security risk.
    pub fn accept_invalid_certs(mut self, accept: bool) -> Self {
        self.reqwest_builder = self.reqwest_builder.danger_accept_invalid_certs(accept);

        self
    }

    /// Sets the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.reqwest_builder = self.reqwest_builder.timeout(timeout);

        self
    }

    /// Sets the endpoint of the client.
    /// The default endpoint is `https://ptr.hydrus.network:45871`
    pub fn endpoint<S: ToString>(mut self, endpoint: S) -> Self {
        self.endpoint = endpoint.to_string();

        self
    }

    /// Sets the access key. This key is required for requests
    /// to the PTR.
    pub fn access_key<S: ToString>(mut self, access_key: S) -> Self {
        self.access_key = Some(access_key.to_string());

        self
    }

    /// Validates the configuration and builds the client
    pub fn build(self) -> Result<Client> {
        let access_key = self
            .access_key
            .ok_or_else(|| Error::Builder(String::from("missing access key")))?;

        Ok(Client {
            client: self.reqwest_builder.build()?,
            base_url: self.endpoint,
            access_key,
        })
    }
}
