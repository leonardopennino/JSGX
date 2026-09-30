use base64::prelude::{Engine as _, BASE64_STANDARD};
use ring::{digest::SHA256, signature::RSA_PKCS1_2048_8192_SHA256};
use std::{fs::File, io::Read};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::error::StartupError;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub capabilities: Vec<String>,
    #[serde(skip)]
    pub pubkey_hash: Vec<u8>,
    #[serde(skip)]
    pub pubkey_data: Vec<u8>,
    #[serde(skip_serializing)]
    pub pubkey: String,
    #[serde(skip_deserializing)]
    pub uuid: Uuid,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub mr_enclave: String,
    pub users: Vec<User>,
}

impl Config {
    pub fn from_config_file(path: &str) -> Result<Self, StartupError> {
        let mut f = File::open(path).map_err(|_| StartupError::ConfigNotFound)?;
        let mut buf = vec![];
        f.read_to_end(&mut buf).unwrap();
        let mut config = toml::from_str::<Config>(std::str::from_utf8(&buf).unwrap())?;
        let users: Result<Vec<User>, StartupError> = config
            .users
            .iter()
            .map(|u| {
                let pubkey_data = BASE64_STANDARD.decode(&u.pubkey)?;
                let pubkey = ring::signature::UnparsedPublicKey::new(
                    &RSA_PKCS1_2048_8192_SHA256,
                    &pubkey_data,
                );
                let pubkey_hash = ring::digest::digest(&SHA256, pubkey.as_ref())
                    .as_ref()
                    .to_vec();
                let uuid = Uuid::new_v4();
                Ok(User {
                    pubkey_data,
                    pubkey_hash,
                    uuid,
                    ..u.clone()
                })
            })
            .collect();
        config.users = users?;
        Ok(config)
    }

    #[cfg(test)]
    pub(crate) fn from_string(str: &str) -> Result<Config, StartupError> {
        let mut config = toml::from_str::<Config>(str)?;
        let users = config
            .users
            .iter()
            .map(|u| {
                let pubkey_data = BASE64_STANDARD.decode(&u.pubkey).unwrap();
                let pubkey_hash = ring::digest::digest(&SHA256, &pubkey_data)
                    .as_ref()
                    .to_vec();
                User {
                    pubkey_data,
                    pubkey_hash,
                    ..u.clone()
                }
            })
            .collect();
        config.users = users;
        Ok(config)
    }
}
#[cfg(test)]
mod tests {
    use crate::{error::StartupError, test::test_resource};

    use super::Config;

    #[test]
    fn config_correctly_parsed() -> Result<(), StartupError> {
        Config::from_config_file(test_resource!("Config_test.toml"))?;
        Ok(())
    }
}
