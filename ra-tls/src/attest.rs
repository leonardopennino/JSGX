use std::{fs::File, io};
use std::io::prelude::*;

use ciborium::Value;
use rcgen::{
    Certificate, CertificateParams, CustomExtension, DistinguishedName, IsCa, KeyPair, SerialNumber,
};
use ring::digest::digest;
use rustls_pki_types::CertificateDer;
use time::OffsetDateTime;

pub const TCG_DICE_TAGGED_EVIDENCE_OID: [u64; 6] = [2,23,133,5,4,9];
pub const TCG_DICE_TAGGED_EVIDENCE_ROW : [u8;6] = [103,129,5,5,4,9];
pub const CERT_SUBJECT_NAME_VALUES: &str = "CN=RATLS,O=SICPA,C=CH";
pub const CERT_TIMESTAMP_NOT_BEFORE_DEFAULT: i128 = 20010101000000;
pub const CERT_TIMESTAMP_NOT_AFTER_DEFAULT: i128 = 20301231235959;

fn create_x509(key_pair: &KeyPair) -> Certificate {
    let evidence = generate_tcg_dice_tagged_evidence(key_pair);
    generate_x509(key_pair, &evidence)
}


fn generate_serialized_pk_hash_entry(key_pair: &KeyPair) -> Vec<u8> {
    let cbor_hash_alg_id = Value::Integer(1.into());
    let sha = digest(&ring::digest::SHA256, key_pair.public_key_raw());
    let cbor_sha = Value::Bytes(sha.as_ref().to_vec());
    let entry = Value::Array(vec!(cbor_hash_alg_id, cbor_sha));
    let mut bytes = vec!();
    ciborium::into_writer(&entry, &mut bytes).unwrap();
    bytes
}

fn generate_serialized_claims(key_pair: &KeyPair) ->Vec<u8> {
    let key = Value::Text("pubkey-hash".into());
    let value = Value::Bytes(generate_serialized_pk_hash_entry(key_pair));
    let item = Value::Map(vec!((key,value)));
    let mut bytes = vec!();
    ciborium::into_writer(&item, &mut bytes).unwrap();
    bytes
}

fn generate_quote_with_claims_hash(claims : &[u8]) -> Vec<u8> {
    let pk_sha = ring::digest::digest(&ring::digest::SHA256, claims);
    write_file("/dev/attestation/user_report_data", &pk_sha.as_ref()).expect("Unable to write user report data");
    let quote = read_file("/dev/attestation/quote").expect("Unable to read data from quoting enclave");
    quote
}
fn combine_quote_and_claims_in_evidence(quote: &[u8], claims : &[u8]) -> Vec<u8> {
    let quote = Value::Bytes(quote.to_vec());
    let claims = Value::Bytes(claims.to_vec());
    let evidence = Value::Array(vec!(quote, claims));
    let tag = Value::Tag(0xEA60,Box::new(evidence));
    let mut bytes = vec!();
    ciborium::into_writer(&tag, &mut bytes).unwrap();
    bytes
}

fn generate_tcg_dice_tagged_evidence(key_pair : &KeyPair) -> Vec<u8> {
    let claims = generate_serialized_claims(key_pair);
    let quote = generate_quote_with_claims_hash(&claims);
    combine_quote_and_claims_in_evidence(&quote, &claims)
}

fn generate_x509(key_pair: &KeyPair, evidence : &Vec<u8>) -> Certificate {
    let alt_names = vec![CERT_SUBJECT_NAME_VALUES.to_string()];
    let mut certificate_params = CertificateParams::new(alt_names).unwrap();
    let mut dn = DistinguishedName::new();
    dn.push(rcgen::DnType::CommonName, "JSGX");
    dn.push(rcgen::DnType::OrganizationName, "SICPA");
    dn.push(rcgen::DnType::CountryName, "CH");
    certificate_params.distinguished_name = dn;
    let serial = &vec![0x01];
    certificate_params.serial_number = Some(SerialNumber::from_slice(&serial));
    certificate_params.not_before =
        OffsetDateTime::from_unix_timestamp_nanos(CERT_TIMESTAMP_NOT_BEFORE_DEFAULT).unwrap();
    certificate_params.not_after =
        OffsetDateTime::from_unix_timestamp_nanos(CERT_TIMESTAMP_NOT_AFTER_DEFAULT).unwrap();
    certificate_params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    let evidence_oid =
        CustomExtension::from_oid_content(&TCG_DICE_TAGGED_EVIDENCE_OID, evidence.to_vec());
    certificate_params.custom_extensions.push(evidence_oid);
    certificate_params.self_signed(key_pair).expect("Error in certificate creation")
}

fn write_file(file: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file = File::options().write(true).open(file)?;
    file.write_all(data)?;
    Ok(())
}
fn read_file(file : &str) -> io::Result<Vec<u8>>{
    let mut file = File::options().read(true).open(file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn create_key_and_crt() -> (KeyPair, String) {
    let key_pair = KeyPair::generate_for(&rcgen::PKCS_ECDSA_P384_SHA384).expect("Unable to generate keys for certificate");
    let cert = create_x509(&key_pair);
    (key_pair, cert.pem().clone())
}
