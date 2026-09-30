use core::time;
use std::process::Stdio;

use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
    process::Command,
};

use crate::{
    config::User,
    error::{StartupError, TeeError},
};

#[derive(Debug, Serialize)]
pub(crate) struct UDSRequest<'a> {
    method: &'a str,
    params: &'a Option<Value>,
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthUDSRequest<'a> {
    method: &'a str,
    params: &'a Option<Value>,
    user: &'a User,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum UDSResponse {
    Success { data: Value },
    Error { error: Value },
}
impl IntoResponse for UDSResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success { data } => (StatusCode::OK, Json(data)).into_response(),
            Self::Error {error }  => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(error),
            )
                .into_response(),
        }
    }
}

impl<'a> UDSRequest<'a> {
    pub fn new(method: &'a str, params: &'a Option<Value>) -> Self {
        Self { method, params }
    }
    pub async fn send(&self) -> Result<impl IntoResponse, TeeError> {
        let mut uds = UnixStream::connect("/tmp/socket")
            .await
            .expect("Could not attach socket");
        let request = serde_json::to_string(&self).unwrap();
        let mut buf = [0; 1024];
        uds.writable().await?;
        uds.write(request.as_bytes()).await?;
        uds.readable().await?;
        let n = uds.read(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[0..n]).to_string();
        let res = serde_json::from_str::<UDSResponse>(&message)
            .map_err(|e| TeeError::UDSFailure(e.to_string()))?;
        Ok(res)
    }
}

impl<'a> AuthUDSRequest<'a> {
    pub fn new(method: &'a str, params: &'a Option<Value>, user: &'a User) -> Self {
        Self {
            method,
            params,
            user,
        }
    }
    pub async fn send(&'a self) -> Result<impl IntoResponse, TeeError> {
        let mut uds = UnixStream::connect("/tmp/socket")
            .await
            .expect("Could not attach socket");
        let request = serde_json::to_string(&self).unwrap();
        let mut buf = [0; 1024];
        uds.writable().await?;
        uds.write(request.as_bytes()).await?;
        uds.readable().await?;
        let n = uds.read(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[0..n]).to_string();
        let res = serde_json::from_str::<Value>(&message)
            .map_err(|_| TeeError::UDSFailure("Failed json deserializing".to_owned()))?;
        Ok(Json(res))
    }
}

pub(crate) async fn initilalize_java_vm() -> Result<(), StartupError> {
    let java = if cfg!(debug_assertions) {
        "java"
    } else {
        "/usr/lib/jvm/java-17-openjdk-amd64/bin/java"
    };
    if !std::path::Path::new("java-tee.jar").exists() {
        Err(StartupError::JarNotFoundError)?;
    }
    let err = if cfg!(debug_assertions) {
        Command::new(java)
            .args(["-jar", "-Xmx1G", "java-tee.jar"])
            .spawn()
    } else {
        Command::new(java)
            .args(["-jar", "-Xmx1G", "java-tee.jar"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    };
    match err {
        Ok(_x) => println!("Java vm initialized"),
        Err(e) => return Err(StartupError::JavaVmInitError(e.to_string())),
    }
    println!("Connecting to java vm...");
    let mut connected = false;
    for _i in 0..10 {
        let result = UnixStream::connect("/tmp/socket").await;
        if result.is_ok() {
            connected = true;
            break;
        }
        std::thread::sleep(time::Duration::from_secs_f32(1.0));
    }
    if !connected {
        Err(StartupError::JavaVmInitError(
            "Unable to init Java vm".to_string(),
        ))?;
    }
    println!("VM Connected");
    Ok(())
}
