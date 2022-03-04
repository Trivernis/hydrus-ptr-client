mod common;

#[tokio::test]
async fn test_options() {
    let client = common::get_client();
    client.get_options().await.unwrap();
}

#[tokio::test]
async fn test_metadata() {
    let client = common::get_client();
    client.get_metadata(0).await.unwrap();
}

const DEFINITIONS_UPDATE_HASH: &str =
    "4a4d13c1fcdf0cf734927ec4c9637fdac6144512ad7dc919e0f222e7b0e71586";
const CONTENT_UPDATE_HASH: &str =
    "cd1418ffeba0b8fe46aefa51a7adf1210356523ead658b182762ff61b73ebae5";

#[tokio::test]
async fn test_update() {
    let client = common::get_client();

    client.get_update(DEFINITIONS_UPDATE_HASH).await.unwrap();
    client.get_update(CONTENT_UPDATE_HASH).await.unwrap();
}
