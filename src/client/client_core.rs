use crate::{Client, Error, FromJson, GetEndpoint};
use flate2::write::ZlibDecoder;
use reqwest::Response;
use serde::Serialize;
use std::fmt::Debug;
use std::io::Write;

impl Client {
    /// Performs a get request to the given Get Endpoint
    #[tracing::instrument(skip(self), level = "debug")]
    pub(crate) async fn get<E: GetEndpoint, Q: Serialize + Debug>(
        &self,
        query: &Q,
    ) -> crate::Result<E::Response> {
        tracing::trace!("GET request to {}", E::path());
        let response = self
            .client
            .get(format!("{}/{}", self.base_url, E::path()))
            .query(query)
            .header("Hydrus-Key", self.access_key.to_string())
            .send()
            .await?;
        let body = Self::get_body(response).await?;
        let bytes = Self::decompress_body(body)?;
        let response_type = Self::deserialize_body(bytes)?;
        tracing::trace!("response is: {:?}", response_type);

        Ok(response_type)
    }

    /// Returns the body from the response
    #[tracing::instrument(level = "trace")]
    async fn get_body(response: Response) -> crate::Result<Vec<u8>> {
        if response.status().is_success() {
            Ok(response.bytes().await?.to_vec())
        } else {
            let message = response.text().await?;
            Err(Error::Response(message))
        }
    }

    /// Uses zlib to decompress the body
    #[tracing::instrument(skip(bytes), level = "trace")]
    fn decompress_body(mut bytes: Vec<u8>) -> crate::Result<Vec<u8>> {
        tracing::trace!("body length {}", bytes.len());

        let mut buf = Vec::new();
        let mut decoder = ZlibDecoder::new(buf);

        decoder.write_all(&mut bytes)?;
        buf = decoder.finish()?;

        tracing::trace!("result length {}", buf.len());

        Ok(buf)
    }

    /// Deserializes the body to the given type
    #[tracing::instrument(skip(bytes), level = "trace")]
    fn deserialize_body<T: FromJson>(bytes: Vec<u8>) -> crate::Result<T> {
        let json_value: serde_json::Value = serde_json::from_reader(&bytes[..])?;
        tracing::trace!("json value = {}", json_value.to_string());

        T::from_json(json_value)
    }
}
