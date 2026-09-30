use std::fs::File;
use std::io::Read;
use std::sync::RwLock;

use crate::auth;
use crate::auth::AuthRequestBody;
use crate::commands::Command;
use reqwest::header::AUTHORIZATION;
use reqwest::Client as RClient;
use reqwest::ClientBuilder;
use ring::rand::SystemRandom;
use ring::rsa::KeyPair;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("The provided file was not found")]
    InvalidFile,
    #[error("The provided file was invalid")]
    ParsingError,
    #[error("You must provide a key file")]
    KeyNotProvided,
    #[error("Error while reading file")]
    IO(#[from] std::io::Error),
}

pub struct Client {
    client: RClient,
    #[cfg(feature = "sgx")]
    params: Option<RaTlsParams>,
    base_url: String,
    auth_data: Option<AuthData>,
}

pub struct AuthData {
    mr_enclave: String,
    key_pair: KeyPair,
    key_hash_encoded: String,
    rand: SystemRandom,
    jwt: RwLock<Option<String>>,
}


impl Client {
    pub (crate) fn new_with_auth(base_url: &str, key_pair: KeyPair, mr_enclave: &str) -> Self {
        use base64::{prelude::BASE64_URL_SAFE, Engine};
        use ring::digest::SHA256;

        let client = ClientBuilder::new()
            .tls_built_in_root_certs(false)
            .build()
            .unwrap();
        let base_url = base_url.to_string();
        let key_digest = ring::digest::digest(&SHA256, key_pair.public().as_ref());
        let key_hash_encoded = BASE64_URL_SAFE.encode(key_digest.as_ref());
        let mr_enclave = mr_enclave.to_string();
        let rand = ring::rand::SystemRandom::new();
        let auth_data = AuthData {key_pair, key_hash_encoded, rand, mr_enclave, jwt : RwLock::new(None)};
        Self {
            client,
            base_url,
            auth_data : Some(auth_data)
        }
    }
    pub  fn new(base_url: &str) -> Self{
        let client = ClientBuilder::new()
            .tls_built_in_root_certs(false)
            .build()
            .unwrap();
        let base_url = base_url.to_string();
        Self {client, base_url, auth_data : None}

    }

    pub async fn call_auth_method(
        &self,
        method_name: &str,
        param: Option<&Value>,
    ) -> Result<Value, String> {
        let authdata = self.auth_data.as_ref().unwrap();
        let jwtref = authdata.jwt.read().unwrap();
        let jwt = jwtref.as_ref().unwrap();

        let response = self
            .client
            .post(format!("{}/ra/{method_name}", self.base_url))
            .json(&param.unwrap_or(&Value::Null))
            .header(AUTHORIZATION, format!("Bearer {}", jwt))
            .send()
            .await
            .unwrap();
        match response.error_for_status_ref() {
            Ok(_x) => Ok(response.json::<Value>().await.unwrap_or(Value::Null)),
            Err(_e) => Err(response.text().await.unwrap()),
        }
    }

    pub async fn call_method(
        &self,
        method_name: &str,
        param: Option<&Value>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(format!("{}/{method_name}", self.base_url))
            .json(param.unwrap_or(&Value::Null))
            .send()
            .await
            .unwrap();
        match response.error_for_status_ref() {
            Ok(_x) => Ok(response.json::<Value>().await.unwrap_or(Value::Null)),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn authenticate(&self) {
        let authdata = self.auth_data.as_ref().unwrap();
        let message = AuthRequestBody::new(&authdata.mr_enclave, &authdata.key_hash_encoded);
        let message = auth::create_signed_message(&message, &authdata.key_pair, &authdata.rand).unwrap();
        let response = self
            .client
            .post(format!("{}/api/authenticate", self.base_url))
            .json(&message)
            .send()
            .await
            .unwrap();
        let token = response.text().await.unwrap();
        let mut jwtref = authdata.jwt.write().unwrap();
        *jwtref = Some(token);
    }
}
impl ClientConfig {
    pub(crate) fn from_config_str(config: &str) -> Result<Client, ConfigError> {
        let config: ClientConfig =
            toml::from_str(&config).map_err(|_| ConfigError::ParsingError)?;
        let key_path = config.key_path.ok_or(ConfigError::KeyNotProvided)?;
        let key = auth::parse_key_from_path(&key_path).unwrap();
        Ok(Client::new(&config.base_url))
    }

    pub fn from_config_file(path: &str) -> Result<Client, ConfigError> {
        let mut file = File::open(path)?;
        let mut buf: String = Default::default();
        file.read_to_string(&mut buf)?;
        Self::from_config_str(&buf)
    }

    pub fn from_config(config: &ClientConfig) -> Result<Client, String> {
        let key_path = config
            .key_path
            .as_ref()
            .ok_or("Key path was not provided")?;
        let key = auth::parse_key_from_path(&key_path).unwrap();
        Ok(Client::new(
            &config.base_url,
        ))
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ClientConfig {
    pub base_url: String,
    pub mr_enclave: String,
    pub key_path: Option<String>,
    pub commands: Vec<Command>,
}
