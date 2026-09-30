use crate::config::{Config, User};
use crate::error::{StartupError, TeeError};
use crate::uds::{AuthUDSRequest, UDSRequest};
use crate::{auth, uds};
use anyhow::Result;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use ring::hmac::{self, HMAC_SHA256};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct ServerState {
    pub config: Config,
    pub jwt_key: hmac::Key,
}
#[allow(dead_code)]
async fn http_server(cert: String) {
    let http = Router::new().route("/certificate", get(|| async { cert }));
    let http_addr = SocketAddr::from(([0, 0, 0, 0], 3333));
    axum_server::bind(http_addr)
        .serve(http.into_make_service())
        .await
        .unwrap();
}

pub async fn serve(app: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    #[cfg(feature = "sgx")]
    {
        use axum_server::tls_rustls::RustlsConfig;
        use ra_tls::attest;

        let cert = attest::create_key_and_crt();
        tokio::spawn(http_server(cert.1.clone()));
        let config = RustlsConfig::from_pem(cert.1.into(), cert.0.serialize_pem().into())
            .await
            .unwrap();
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
    #[cfg(not(feature = "sgx"))]
    {
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

pub async fn create_router_with_config(config: Config) -> Result<Router, StartupError> {
    let rng = ring::rand::SystemRandom::new();
    let jwt_key = hmac::Key::generate(HMAC_SHA256, &rng)
        .map_err(|_| StartupError::UnknownError("Unable to generate random hmac key".into()))?;
    let state = Arc::new(ServerState { config, jwt_key });
    let app = Router::new()
        .route("/api/authenticate", post(auth::login))
        .route("/:method", post(handler))
        .route("/ra/:method", post(authenticated_handler))
        .route("/", post(handler))
        .with_state(state);
    uds::initilalize_java_vm().await.unwrap();
    Ok(app)
}

async fn authenticated_handler(
    user: User,
    Path(params): Path<Vec<String>>,
    data: Option<Json<Value>>,
) -> impl IntoResponse {
    if params.len() == 0 {
        return Err(TeeError::NoMethodSpecified);
    }
    let method: String = match params.get(0) {
        Some(x) => x.to_string(),
        None => return Err(TeeError::NoMethodSpecified),
    };
    let params = data.map_or(None, |v| Some(v.0));
    let request = AuthUDSRequest::new(&method, &params, &user);
    let result = request.send().await?;
    Ok(result)
}

async fn handler(Path(params): Path<Vec<String>>, data: Option<Json<Value>>) -> impl IntoResponse {
    if params.len() == 0 {
        return Err(TeeError::NoMethodSpecified);
    }
    let method: String = match params.get(0) {
        Some(x) => x.to_string(),
        None => return Err(TeeError::NoMethodSpecified),
    };
    let params = data.map_or(None, |v| Some(v.0));
    let request = UDSRequest::new(&method, &params);
    let result = request.send().await?;
    Ok(result)
}

#[cfg(test)]
mod tests {

    use crate::{config::Config, test::test_resource};
    use axum_test::TestServer;

    #[tokio::test]
    pub async fn test_server_starts_up() -> Result<(), Box<dyn std::error::Error>> {
        let app = super::create_router_with_config(Config::from_config_file(test_resource!(
            "Config_test.toml"
        ))?)
        .await?;
        let _server = TestServer::new(app).unwrap();

        Ok(())
    }
}
