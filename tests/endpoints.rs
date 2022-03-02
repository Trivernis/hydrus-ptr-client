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

#[tokio::test]
async fn test_update() {
    let client = common::get_client();
    client
        .update("4a4d13c1fcdf0cf734927ec4c9637fdac6144512ad7dc919e0f222e7b0e71586")
        .await
        .unwrap();
}
