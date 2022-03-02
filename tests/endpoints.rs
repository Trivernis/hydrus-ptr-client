mod common;

#[tokio::test]
async fn test_options() {
    let client = common::get_client();
    client.options().await.unwrap();
}

#[tokio::test]
async fn test_metadata() {
    let client = common::get_client();
    client.metadata(0).await.unwrap();
}
