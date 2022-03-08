# Hydrus PTR Client [![](https://img.shields.io/docsrs/hydrus-ptr-client)](https://docs.rs/hydrus-ptr-client) [![](https://img.shields.io/crates/v/hydrus-ptr-client)](https://crates.io/crates/hydrus-ptr-client)

A rust http client for the hydrus PTR. Completeness is not guaranteed.

## Usage

## Fetching metadata and retrieving updates

```rust
use hydrus_ptr_client::Client;

#[tokio::main]
async fn main() {
    let client = Client::builder().accept_invalid_certs(true).build().unwrap();
    // list of all update files since id = 0
    let metadata = client.get_metadata(0).await.unwrap();
    let first_update_file = metadata.update_hashes().swap_remove(0);
    let update = client.get_update(first_update_file).await.unwrap();
    println!("Got update {:?}", update);
}
```

## Streaming updates

```rust
use hydrus_ptr_client::Client;
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let client = Client::builder().accept_invalid_certs(true).build().unwrap();
    // streams all update since id = 0
    let mut update_stream = client.stream_updates(0).await.unwrap();
    
    while let Some(result) = update_stream.next().await {
        match result {
            Ok(update) => println!("We got an update {:?}", update),
            Err(e) => println!("Oh no, an error occurred {}", e),
        }
        break;
    }
}
```

## License

Apache-2.0