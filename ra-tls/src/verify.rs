use crate::error::{AttestationResult, RATLSError};
use crate::maa_client::{
    JWTResponseToken, JWTTokenPayload, MaaCert, MaaClient, MAA_URL_CERTS_ENDPOINT,
};
use crate::{
    error::InvalidPayloadEntry,
    sgx_types::{self, sgx_quote_t},
};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};
use bcder::ConstOid;
use ciborium::Value;
use ring::digest::digest;
use ring::signature::UnparsedPublicKey;
use std::fs::File;
use std::io::Write;
use x509_certificate::X509Certificate;

pub struct RaTlsConfig {
    parameters: EnclaveParameters,
    certificate: X509Certificate,
}
pub struct EnclaveParameters {
    pub maa_base_url: String,
    pub maa_provider_api_version: String,
    pub mr_signer: String,
    pub mr_enclave: String,
    pub product_id: u32,
    pub security_version: u32,
}
impl RaTlsConfig {
    pub fn parameters(&self) -> &EnclaveParameters {
        &self.parameters
    }
    pub fn new(parameters: EnclaveParameters, certificate: String) -> AttestationResult<Self> {
        let certificate = X509Certificate::from_pem(certificate)
            .map_err(|_| RATLSError::InvalidCertificateProvided)?;
        Ok(Self {
            certificate,
            parameters,
        })
    }

    pub async fn ra_tls_verify(&self) -> Result<(), RATLSError> {
        if !self.certificate.subject_is_issuer() {
            panic!("Expected a self signed certificate");
        }
        let quote = extract_quote_and_pubkey(&self.certificate)?;
        let (head, body, _tail) = unsafe { quote.align_to::<sgx_types::sgx_quote_t>() };
        assert!(head.is_empty(), "Data was not aligned");
        let sgx_quote = &body[0];
        let cert_pkey = self.certificate.public_key_data();
        cmp_crt_pk_against_quote_report_data(&self.certificate, sgx_quote)?;
        let data = generate_serialized_claims(&cert_pkey);
        let client = MaaClient::new(&self.parameters.maa_base_url, &self.parameters.maa_provider_api_version);
        let jwk = client.maa_get_signing_certs().await.unwrap();
        let data_encoded = URL_SAFE.encode(data);
        let quote_encoded = URL_SAFE.encode(quote);
        let res = client
            .maa_send_request(&quote_encoded, &data_encoded)
            .await
            .unwrap();
        maa_verify_response_output_quote(&res, &jwk, &self.parameters)?;
        println!("Verification completed");
        Ok(())
    }
}

fn maa_verify_response_output_quote<'a>(
    maa_response: &JWTResponseToken,
    signing_cert: &MaaCert,
    config: &'a EnclaveParameters,
) -> AttestationResult<()> {
    validate_token_claims(&maa_response, config, signing_cert).unwrap();
    verify_quote_body_enclave_attributes(&config, &maa_response.body.decoded)?;
    Ok(())
}

fn validate_token_claims<'a>(
    token: &JWTResponseToken,
    config: &EnclaveParameters,
    cert: &MaaCert,
) -> AttestationResult<()> {
    let header = &token.header.decoded;
    if header.alg != "RS256" || header.typ != "JWT" || header.kid.len() == 0 {
        return Err(RATLSError::GenericError("Error in jwt header".to_string()));
    }
    if header.jku != format!("{}/{}", config.maa_base_url, MAA_URL_CERTS_ENDPOINT) {
        return Err(RATLSError::GenericError(format!(
            "JWT signed by different entity, expected {}{}:, found {}:",
            config.maa_base_url, MAA_URL_CERTS_ENDPOINT, header.jku
        )));
    }
    if cert.keys.iter().any(|k| k.kty != "RSA") {
        return Err(RATLSError::GenericError(
            "MAA JWS kty is not RSA".to_string(),
        ));
    };
    let signing_cert = cert
        .keys
        .iter()
        .find(|k| k.kid == header.kid)
        .ok_or(RATLSError::GenericError(
            "Could not find equal kid".to_string(),
        ))?
        .x5c
        .get(0)
        .ok_or(RATLSError::GenericError(
            "Could not find first certificate in x5c".to_string(),
        ))?;

    verify_certificate_signature(
        signing_cert,
        &format!(
            "{}.{}",
            token.header.entry.encoded, token.body.entry.encoded
        ),
        &token.signature.data,
    )
    .unwrap();
    verify_payload(&token.body.decoded).unwrap();

    Ok(())
}
fn verify_field<'a, 'b, T>(
    actual: &'a T,
    expected: &'b T,
    field_name: &str,
) -> AttestationResult<()>
where
    T: std::cmp::PartialEq + std::fmt::Debug + std::fmt::Display + Clone + 'static,
{
    if actual != expected {
        let entry = InvalidPayloadEntry::new(
            Box::new(expected.clone()),
            Box::new(actual.clone()),
            field_name,
        );
        return Err(RATLSError::InvalidPayloadEntryError(Box::new(entry)));
    }
    Ok(())
}
fn verify_payload(payload: &JWTTokenPayload) -> AttestationResult<()> {
    verify_field(
        &payload.x_ms_attestation_type,
        &"sgx".to_string(),
        "Attestation type",
    )?;
    verify_field(&payload.x_ms_ver, &"1.0".to_string(), "Version")?;
    let current_time = chrono::Utc::now();
    if payload.nbf < current_time && current_time < payload.exp {
        return Err(RATLSError::TokenExpired);
    }

    Ok(())
}

fn verify_certificate_signature<'a>(
    signing_cert_b64: &str,
    payload: &str,
    signature: &Vec<u8>,
) -> AttestationResult<()> {
    let decoded = STANDARD.decode(signing_cert_b64).unwrap();
    let crt_der = x509_parser::parse_x509_certificate(&decoded).unwrap().1;
    let pubkey = crt_der.public_key().clone().subject_public_key.data;
    let key = UnparsedPublicKey::new(&ring::signature::RSA_PKCS1_2048_8192_SHA256, pubkey);
    key.verify(payload.as_bytes(), &signature).unwrap();
    Ok(())
}

fn extract_quote_and_pubkey(sgx_cert: &X509Certificate) -> Result<Vec<u8>, RATLSError> {
    let oid = ConstOid {
        0: &[103, 129, 5, 5, 4, 9],
    };
    let extension = sgx_cert
        .iter_extensions()
        .find(|e| e.id.eq(&oid))
        .ok_or(RATLSError::QuoteOidNotFound)?;
    let bytes: &[u8] = &extension.value.to_bytes();
    let val = ciborium::from_reader::<ciborium::Value, &[u8]>(bytes)?;
    let tag = val
        .into_tag()
        .map_err(|e| RATLSError::from_cbor_value_err(&e, &Value::Tag(0, Box::new(Value::Null))))?;
    let arr = tag
        .1
        .into_array()
        .map_err(|e| RATLSError::from_cbor_value_err(&e, &Value::Array(vec![])))?;

    let quote = arr
        .get(0)
        .ok_or(RATLSError::GenericError(
            "Missing quote in CBOR array at index 0".to_string(),
        ))?
        .clone()
        .into_bytes()
        .map_err(|e| RATLSError::from_cbor_value_err(&e, &Value::Bytes(vec![])))?;

    // only for testing
    let mut f = File::options()
        .write(true)
        .create(true)
        .open("quote.dat")
        .unwrap();
    f.write_all(&quote).unwrap();
    // ----------------
    Ok(quote)
}

fn cmp_crt_pk_against_quote_report_data<'a>(
    cert: &X509Certificate,
    quote: &sgx_quote_t,
) -> AttestationResult<()> {
    let expected_bytes = generate_serialized_claims(&cert.public_key_data());
    let digest = digest(&ring::digest::SHA256, &expected_bytes);
    match quote.body.report_body.report_data.d[0..32].eq(digest.as_ref()) {
        true => Ok(()),
        false => Err(RATLSError::GenericError(
            "Report data and certificate public key don't match".to_string(),
        )),
    }
}

fn generate_serialized_pk_hash_entry(pubkey: &[u8]) -> Vec<u8> {
    let cbor_hash_alg_id = Value::Integer(1.into());
    let sha = digest(&ring::digest::SHA256, pubkey);
    let cbor_sha = Value::Bytes(sha.as_ref().to_vec());
    let entry = Value::Array(vec![cbor_hash_alg_id, cbor_sha]);
    let mut bytes = vec![];
    ciborium::into_writer(&entry, &mut bytes).unwrap();
    bytes
}

fn generate_serialized_claims(pubkey: &[u8]) -> Vec<u8> {
    let key = Value::Text("pubkey-hash".into());
    let value = Value::Bytes(generate_serialized_pk_hash_entry(pubkey));
    let item = Value::Map(vec![(key, value)]);
    let mut bytes = vec![];
    ciborium::into_writer(&item, &mut bytes).unwrap();
    bytes
}

fn verify_quote_body_enclave_attributes(
    config: &EnclaveParameters,
    payload: &JWTTokenPayload,
) -> AttestationResult<()> {
    verify_field(&config.mr_signer, &payload.x_ms_sgx_mrsigner, "MRSIGNER")?;
    verify_field(&config.mr_enclave, &payload.x_ms_sgx_mrenclave, "MRENCLAVE")?;
    verify_field(
        &config.product_id,
        &payload.x_ms_sgx_product_id,
        "PRODUCTID",
    )?;
    verify_field(&config.security_version, &payload.x_ms_sgx_svn, "ISVSVN")?;
    Ok(())
}
