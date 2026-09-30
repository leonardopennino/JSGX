use anyhow::Result;
use rust_ta::{config::Config, server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_config_file("./resources/tests/Config_test.toml").unwrap();
    let app = server::create_router_with_config(config).await.unwrap();
    server::serve(app).await;
    Ok(())
}
