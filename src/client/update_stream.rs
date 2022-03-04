use crate::Result;
use crate::{Client, UpdateResponse};
use futures_core::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

impl Client {
    pub async fn stream_updates(&self, since: u64) -> Result<UpdateStream> {
        let entries = self.get_metadata(since).await?.0.entries;
        let hashes = entries
            .into_iter()
            .flat_map(|e| e.update_hashes)
            .collect::<Vec<String>>();
        let client = self.clone();

        Ok(UpdateStream::new(client, hashes))
    }
}

/// A stream of update files
/// Used like follows:
///
/// ```
///
/// # use hydrus_ptr_client::{Client};
/// use futures_util::StreamExt;
///
/// # async fn a() {///
/// # let client = Client::new("", "");
/// let mut stream = client.stream_updates(0).await.unwrap();
///
/// while let Some(Ok(update)) = stream.next().await {
///     // do something
/// }
/// # }
/// ```
pub struct UpdateStream {
    failed_hashes: Vec<String>,
    pending_hash: Option<String>,
    hashes: Vec<String>,
    client: Option<Client>,
    fut: Option<Pin<Box<dyn Future<Output = (Result<UpdateResponse>, Client)>>>>,
}

impl UpdateStream {
    pub(crate) fn new(client: Client, mut hashes: Vec<String>) -> Self {
        hashes.reverse();

        Self {
            client: Some(client),
            hashes,
            fut: None,
            failed_hashes: Vec::new(),
            pending_hash: None,
        }
    }

    /// Re-enqueues the latest failed hash for retry
    pub fn retry_latest(&mut self) {
        if let Some(hash) = self.failed_hashes.pop() {
            self.hashes.push(hash);
        }
    }

    /// Puts all failed hashes back into the queue for retry
    pub fn retry_all(&mut self) {
        self.hashes.append(&mut self.failed_hashes);
    }

    /// Returns a list of all failed hashes
    pub fn failed_hashes(&self) -> &Vec<String> {
        &self.failed_hashes
    }
}

impl Unpin for UpdateStream {}

impl Stream for UpdateStream {
    type Item = Result<UpdateResponse>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.fut.is_none() {
            if self.hashes.is_empty() {
                return Poll::Ready(None);
            }
            let hash = self.hashes.pop().unwrap();
            self.pending_hash = Some(hash.clone());
            let client = self.client.take().unwrap();

            self.fut = Some(Box::pin(async move {
                let update = client.get_update(hash).await;

                (update, client)
            }));
        }

        match self.fut.as_mut().unwrap().as_mut().poll(cx) {
            Poll::Ready((result, client)) => {
                self.client = Some(client);
                self.fut = None;

                if result.is_err() {
                    let pending_hash = self.pending_hash.take().unwrap();
                    self.failed_hashes.push(pending_hash);
                }

                Poll::Ready(Some(result))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
