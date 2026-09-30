pub mod attest;
pub mod verify;
mod sgx_types;
mod maa_client;
pub mod error;
//#[tokio::main]
//async fn main() {
//    let client = verify::init();
//    let mut cert_pem : Vec<u8>  = vec!();
//    File::open("cert.pem").expect("Certificate not found").read_to_end(&mut cert_pem).unwrap();
//    let cert = X509Certificate::from_pem(&cert_pem).unwrap();
//    cert.iter_extensions().for_each(|e| println!("{}", e.id));
//    println!("{}", verify::maa_get_signing_certs(&client).await.unwrap());
//    verify::ra_tls_verify(&cert).await.unwrap();
//}
