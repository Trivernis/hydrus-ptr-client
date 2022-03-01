mod common;

#[tokio::test]
async fn test_options() {
    let client = common::get_client();
    client.options().await.unwrap();
}
