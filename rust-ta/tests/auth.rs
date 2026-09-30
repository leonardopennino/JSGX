use axum_test::TestServer;
use rust_ta::{config::Config, server};

macro_rules! test_resource {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/resources/tests/", $fname)
    };
}

#[tokio::test]
pub async fn test_server_starts_up() -> Result<(), Box<dyn std::error::Error>> {
    let app = server::create_router_with_config(Config::from_config_file(test_resource!(
        "Config_test.toml"
    ))?)
    .await?;
    let _server = TestServer::new(app)?;
    Ok(())
}
fn create_jwt() -> String {
    todo!()
}
