use crate::error::{AuthError, AuthResult};
use crate::{
    config::{Config, User},
    server::ServerState,
};
use axum::{
    async_trait,
    extract::{FromRequestParts, Json, State},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use base64::prelude::{Engine as _, BASE64_URL_SAFE};
use reqwest::header::AUTHORIZATION;
use ring::{
    hmac::{self, Key},
    rand::SecureRandom,
    rsa::KeyPair,
    signature::RSA_PKCS1_2048_8192_SHA256,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AuthRequestBody {
    mr_enclave: String,
    timestamp: u64,
    pub_key_hash_encoded: String,
}

impl AuthRequestBody {
    pub(crate) fn new(mr_enclave: &str, pub_key_hash_encoded: &str) -> Self {
        let mr_enclave = mr_enclave.to_string();
        let pub_key_hash_encoded = pub_key_hash_encoded.to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            mr_enclave,
            pub_key_hash_encoded,
            timestamp,
        }
    }
}

pub(crate) struct B64Entry {
    pub data: Vec<u8>,
}

impl B64Entry {
    pub fn new(
        encoded_str: &str,
        encoder: Option<base64::engine::GeneralPurpose>,
    ) -> AuthResult<Self> {
        let enc = encoder.unwrap_or(BASE64_URL_SAFE);
        let data = enc.decode(encoded_str)?;
        Ok(Self { data })
    }
}

pub(crate) struct JB64Entry<T> {
    pub decoded: T,
}

impl<T> JB64Entry<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn new(
        encoded_str: &str,
        encoder: Option<base64::engine::GeneralPurpose>,
    ) -> AuthResult<Self> {
        let entry = B64Entry::new(encoded_str, encoder)?;
        let decoded = serde_json::from_str::<T>(&std::str::from_utf8(&entry.data).unwrap())?;
        Ok(Self { decoded })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthRequest {
    body: String,
    signature: String,
}

pub(crate) async fn login(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<AuthRequest>,
) -> AuthResult<String> {
    let user = verify_signature_and_extract_user(&request, &state.config)?;
    let jwt = create_jwt(&state.jwt_key, &user);
    Ok(jwt)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct JwtToken {
    uuid: Uuid,
    iat: u64,
}

fn create_jwt(jwt_key: &Key, user: &User) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let token = JwtToken {
        uuid: user.uuid.clone(),
        iat: timestamp,
    };
    let token = BASE64_URL_SAFE.encode(serde_json::to_string(&token).unwrap());
    let tag = hmac::sign(&jwt_key, &token.as_bytes());
    let signature = BASE64_URL_SAFE.encode(tag);
    format!("{token}.{signature}")
}

#[async_trait]
impl FromRequestParts<Arc<ServerState>> for User {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ServerState>,
    ) -> Result<Self, Self::Rejection> {
        if let Some(jwt_token) = parts.headers.get(AUTHORIZATION) {
            let token = jwt_token
                .to_str()
                .map_err(|_| AuthError::INVALIDJWT.into_response())?
                .to_owned();
            let token = &token["Bearer ".len()..];
            Ok(
                verify_jwt_and_extract_user(&token, &state.jwt_key, &state.config)
                    .map_err(|err| err.into_response())?
                    .clone(),
            )
        } else {
            Err(AuthError::MISSINGJWT.into_response())
        }
    }
}

pub(crate) fn verify_jwt_and_extract_user<'a, 'b>(
    jwt: &str,
    jwt_key: &'a Key,
    config: &'a Config,
) -> AuthResult<&'b User>
where
    'a: 'b,
{
    let middle = jwt.find(|c| c == '.').ok_or(AuthError::INVALIDJWT)?;
    let (body, signature) = jwt.split_at_checked(middle).ok_or(AuthError::INVALIDJWT)?;
    let signature = BASE64_URL_SAFE.decode(&signature[1..])?; // The first byte is the "." and has to be skipped
    hmac::verify(&jwt_key, &body.as_bytes(), &signature).map_err(|_| AuthError::INVALIDJWT)?;
    let body: JB64Entry<JwtToken> = JB64Entry::new(&body, None)?;
    config
        .users
        .iter()
        .find(|u| u.uuid == body.decoded.uuid)
        .ok_or(AuthError::USERNOTFOUNDERROR)
}

fn verify_signature_and_extract_user<'a, 'b>(
    request: &AuthRequest,
    config: &'a Config,
) -> Result<&'b User, AuthError>
where
    'a: 'b,
{
    let body: JB64Entry<AuthRequestBody> = JB64Entry::new(&request.body, None)?;
    let user = extract_user_from_pkey(&body.decoded.pub_key_hash_encoded, &config.users)?;
    let verifier =
        ring::signature::UnparsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, &user.pubkey_data);
    let signature = B64Entry::new(&request.signature, None)?;
    verifier.verify(request.body.as_bytes(), &signature.data)?;
    if body.decoded.mr_enclave != config.mr_enclave {
        Err(AuthError::INVALIDMRENCLAVE)?;
    }
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if current_timestamp - body.decoded.timestamp > 5 {
        Err(AuthError::INVALIDTIMESTAMP)?;
    }
    Ok(user)
}

fn extract_user_from_pkey<'a>(pkey_hash: &str, users: &'a [User]) -> AuthResult<&'a User> {
    let hash = BASE64_URL_SAFE.decode(pkey_hash)?;
    let user = users
        .iter()
        .find(|u| u.pubkey_hash == hash)
        .ok_or(AuthError::USERNOTFOUNDERROR)?;
    Ok(&user)
}

pub(crate) fn create_signed_message(
    body: &AuthRequestBody,
    key_pair: &KeyPair,
    rand: &dyn SecureRandom,
) -> Result<AuthRequest, Box<dyn std::error::Error>> {
    let body_json = serde_json::json!(&body);
    let body_encoded = BASE64_URL_SAFE.encode(&body_json.to_string());
    let mut signature = vec![0; key_pair.public().modulus_len()];
    key_pair
        .sign(
            &ring::signature::RSA_PKCS1_SHA256,
            rand,
            &body_encoded.as_bytes(),
            &mut signature,
        )
        .unwrap();
    Ok(AuthRequest {
        body: body_encoded,
        signature: BASE64_URL_SAFE.encode(signature),
    })
}

fn read_key(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut bytes = vec![];
    f.read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub fn parse_key_from_path(path: &str) -> Result<KeyPair, Box<dyn std::error::Error>> {
    Ok(ring::rsa::KeyPair::from_der(&read_key(path)?).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::auth::create_signed_message;
    use crate::config::Config;
    use crate::error::AuthError;
    use crate::test::test_resource;
    use base64::prelude::{Engine as _, BASE64_URL_SAFE};
    use ring::digest::{Digest, SHA256};
    use ring::hmac::HMAC_SHA256;
    use ring::rand::SystemRandom;
    use ring::rsa::KeyPair;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{read_key, AuthRequestBody};
    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn create_test_config() -> Config {
        Config::from_config_file(&test_resource!("Config_test.toml")).unwrap()
    }

    #[test]
    fn test_correct_jwt_returns_user() -> TestResult {
        let (key_pair, key_digest) = get_test_keypair(test_resource!("key_private.der"))?;
        let request = AuthRequestBody {
            mr_enclave: "TSTMRENCLAVE".to_string(),
            pub_key_hash_encoded: BASE64_URL_SAFE.encode(key_digest.as_ref()),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        let rng = ring::rand::SystemRandom::new();
        let request = create_signed_message(&request, &key_pair, &rng).unwrap();
        let jwt_key = ring::hmac::Key::generate(HMAC_SHA256, &rng).unwrap();
        let config = create_test_config();
        let user = super::verify_signature_and_extract_user(&request, &config).unwrap();
        let jwt = super::create_jwt(&jwt_key, &user);
        super::verify_jwt_and_extract_user(&jwt, &jwt_key, &config)?;
        Ok(())
    }

    #[test]
    fn signature_with_old_timestamp_returns_err() -> TestResult {
        let (key_pair, key_digest) = get_test_keypair(test_resource!("key_private.der"))?;
        let request = AuthRequestBody {
            mr_enclave: "TSTMRENCLAVE".to_string(),
            pub_key_hash_encoded: BASE64_URL_SAFE.encode(key_digest.as_ref()),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - 10,
        };
        let rand = SystemRandom::new();
        let request = create_signed_message(&request, &key_pair, &rand).unwrap();
        let err =
            super::verify_signature_and_extract_user(&request, &create_test_config()).unwrap_err();
        assert!(matches!(err, AuthError::INVALIDTIMESTAMP));
        Ok(())
    }
    #[test]
    fn signature_with_unknown_mr_enclave_returns_err() -> TestResult {
        let (key_pair, key_digest) = get_test_keypair(test_resource!("key_private.der"))?;
        let request = AuthRequestBody {
            mr_enclave: "WRONGMRENCLAVE".to_string(),
            pub_key_hash_encoded: BASE64_URL_SAFE.encode(key_digest.as_ref()),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - 10,
        };
        let rand = SystemRandom::new();
        let request = create_signed_message(&request, &key_pair, &rand).unwrap();
        let err =
            super::verify_signature_and_extract_user(&request, &create_test_config()).unwrap_err();
        assert!(matches!(err, AuthError::INVALIDMRENCLAVE));
        Ok(())
    }

    #[test]
    fn signature_with_unknown_hash_returns_err() -> TestResult {
        let (key_pair, key_digest) = get_test_keypair(test_resource!("unused_private.der"))?;
        let request = AuthRequestBody {
            mr_enclave: "TSTMRENCLAVE".to_string(),
            pub_key_hash_encoded: BASE64_URL_SAFE.encode(key_digest.as_ref()),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - 10,
        };
        let rand = SystemRandom::new();
        let request = create_signed_message(&request, &key_pair, &rand).unwrap();
        let err =
            super::verify_signature_and_extract_user(&request, &create_test_config()).unwrap_err();
        assert!(matches!(err, AuthError::USERNOTFOUNDERROR));
        Ok(())
    }

    fn get_test_keypair(key_path: &str) -> Result<(KeyPair, Digest), Box<dyn std::error::Error>> {
        let key_data = read_key(key_path)?;
        let key_pair = ring::rsa::KeyPair::from_der(&key_data).unwrap();
        let key_digest = ring::digest::digest(&SHA256, key_pair.public().as_ref());
        Ok((key_pair, key_digest))
    }
    #[test]
    fn verify_correct_signature() -> Result<(), Box<dyn std::error::Error>> {
        let (key_pair, key_digest) = get_test_keypair(test_resource!("key_private.der"))?;
        let request = AuthRequestBody {
            mr_enclave: "TSTMRENCLAVE".to_string(),
            pub_key_hash_encoded: BASE64_URL_SAFE.encode(key_digest.as_ref()),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        let rand = SystemRandom::new();
        let request = create_signed_message(&request, &key_pair, &rand).unwrap();
        super::verify_signature_and_extract_user(&request, &create_test_config())?;
        Ok(())
    }
}
