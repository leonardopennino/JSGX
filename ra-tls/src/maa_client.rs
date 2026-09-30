use base64::{engine::general_purpose::{URL_SAFE, STANDARD_NO_PAD, URL_SAFE_NO_PAD}, Engine as _};
use chrono::serde::ts_seconds;
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

use crate::error::{AttestationResult, RATLSError};

pub const MAA_URL_CERTS_ENDPOINT: &str = "certs";
pub const MAA_URL_ATTEST_ENDPOINT: &str = "/attest/SgxEnclave?api-version=";

pub struct B64Entry {
    pub encoded: String,
    pub data: Vec<u8>,
}

impl B64Entry {
    pub fn new(
        encoded_str: &str,
        encoder: Option<base64::engine::GeneralPurpose>,
    ) -> AttestationResult<Self> {
        let enc = encoder.unwrap_or(URL_SAFE);
        let data = enc
            .decode(encoded_str)
            .map_err(|_| RATLSError::Base64DecodeError(encoded_str.to_string()))?;
        Ok(Self {
            encoded: encoded_str.to_string(),
            data,
        })
    }
}

pub struct JB64Entry<T> {
    pub entry: B64Entry,
    pub decoded: T,
}
impl<T> JB64Entry<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn new(
        encoded_str: &str,
        encoder: Option<base64::engine::GeneralPurpose>,
    ) -> AttestationResult<Self> {
        let entry = B64Entry::new(encoded_str, encoder)?;
        let decoded = serde_json::from_str::<T>(&std::str::from_utf8(&entry.data).unwrap())?;
        Ok(Self { entry, decoded })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaaResponse {
    pub token: String,
}

pub struct JWTResponseToken {
    pub header: JB64Entry<JWTTokenHeader>,
    pub body: JB64Entry<JWTTokenPayload>,
    pub signature: B64Entry,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaaCert {
    pub keys: Vec<MaaKey>,
}

impl Display for MaaCert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Keys : {}", self.keys.len()))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JWTTokenHeader {
    pub alg: String,
    pub kid: String,
    pub typ: String,
    pub jku: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct JWTTokenPayload {
    pub x_ms_ver: String,
    pub x_ms_attestation_type: String,
    #[serde(with = "ts_seconds")]
    pub exp: chrono::DateTime<chrono::Utc>,
    #[serde(with = "ts_seconds")]
    pub nbf: chrono::DateTime<chrono::Utc>,
    pub x_ms_sgx_is_debuggable: bool,
    pub x_ms_sgx_mrenclave: String,
    pub x_ms_sgx_mrsigner: String,
    pub x_ms_sgx_product_id: u32,
    pub x_ms_sgx_svn: u32,
    pub x_ms_sgx_report_data: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MaaKey {
    pub alg: String,
    pub kty: String,
    pub r#use: String,
    pub x5c: Vec<String>,
    pub n: String,
    pub e: String,
    pub kid: String,
}

pub struct MaaClient {
    pub maa_base_url: String,
    pub maa_provider_api_version: String,
    client: Client,
}

impl MaaClient {
    pub fn new(maa_base_url: &str, maa_provider_api_version: &str) -> Self {
        let mut header_map = HeaderMap::new();
        header_map.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        Self {
            maa_base_url: maa_base_url.to_string(),
            maa_provider_api_version: maa_provider_api_version.to_string(),
            client,
        }
    }
    pub async fn maa_get_signing_certs(self: &Self) -> AttestationResult<MaaCert> {
        let url = format!("{}/{MAA_URL_CERTS_ENDPOINT}", self.maa_base_url);
        return match self.client.get(url).send().await {
            Ok(x) => match x.error_for_status_ref() {
                Ok(_) => Ok(x.json::<MaaCert>().await.unwrap()),
                Err(err) => Err(RATLSError::GenericError(err.to_string())),
            },
            Err(e) => {
                return Err(RATLSError::GenericError(format!(
                    "Error in request: {:?} - {}",
                    e.status(),
                    e.to_string()
                )))
            }
        };
    }
    pub async fn maa_send_request(
        self: &Self,
        encoded_quote: &str,
        encoded_runtime_data: &str,
    ) -> AttestationResult<JWTResponseToken> {
        let request = json!({ "quote" : encoded_quote, "runtimeData" : { "data": encoded_runtime_data, "dataType" : "Binary" } });
        let res = self
            .client
            .post(format!(
                "{}{MAA_URL_ATTEST_ENDPOINT}{}",
                self.maa_base_url, self.maa_provider_api_version
            ))
            .json(&request)
            .send()
            .await
            .map_err(|e| RATLSError::GenericError(e.to_string()))?;
        match res.error_for_status_ref() {
            Ok(_) => {
                let data = res
                    .json::<MaaResponse>()
                    .await
                    .map_err(|e| RATLSError::GenericError(e.to_string()))?;
                let [header, payload, signature]: [&str; 3] = data
                    .token
                    .split(".")
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap();
                println!("A");
                let header = JB64Entry::new(header, Some(STANDARD_NO_PAD)).unwrap();
                println!("B");
                let body = JB64Entry::new(payload, Some(STANDARD_NO_PAD)).unwrap();
                println!("C");
                let signature = B64Entry::new(signature, Some(URL_SAFE_NO_PAD)).unwrap();
                Ok(JWTResponseToken {
                    header,
                    body,
                    signature,
                })
            }
            Err(_err) => Err(RATLSError::GenericError(res.text().await.unwrap())),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_MAA_CERT: &str = r###"
{
    "keys" : [
    {
        "alg": "RS256",
        "kty": "RSA",
        "use": "sig",
        "x5c": ["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R1ZzSUVOdmNuQnZjbUYwYVc5dU1SUXdFZ1lEVlFRSERBdFRZVzUwWVNCRGJHRnlZVEVMTUFrR0ExVUUKQ0F3Q1EwRXhDekFKQmdOVkJBWVRBbFZUTUI0WERUSTBNRFl5TURFM05Ua3dNRm9YRFRNeE1EWXlNREUzTlRrdwpNRm93Y0RFaU1DQUdBMVVFQXd3WlNXNTBaV3dnVTBkWUlGQkRTeUJEWlhKMGFXWnBZMkYwWlRFYU1CZ0dBMVVFCkNnd1JTVzUwWld3Z1EyOXljRzl5WVhScGIyNHhGREFTQmdOVkJBY01DMU5oYm5SaElFTnNZWEpoTVFzd0NRWUQKVlFRSURBSkRRVEVMTUFrR0ExVUVCaE1DVlZNd1dUQVRCZ2NxaGtqT1BRSUJCZ2dxaGtqT1BRTUJCd05DQUFRMQp2eVR0NWQ0ZndqOEFnQnFFN3hTcithQW1WR0pvTTFSaHRFM2pmRjd5bWdJbmplMndBWkNvY0ZjRFczVDdjb3FaClljY3dwQWNwYWpGR0ZQTHVpQlovbzRJRERqQ0NBd293SHdZRFZSMGpCQmd3Rm9BVWxXOWR6YjBiNGVsQVNjblUKOURQT0FWY0wzbFF3YXdZRFZSMGZCR1F3WWpCZ29GNmdYSVphYUhSMGNITTZMeTloY0drdWRISjFjM1JsWkhObApjblpwWTJWekxtbHVkR1ZzTG1OdmJTOXpaM2d2WTJWeWRHbG1hV05oZEdsdmJpOTJNeTl3WTJ0amNtdy9ZMkU5CmNHeGhkR1p2Y20wbVpXNWpiMlJwYm1jOVpHVnlNQjBHQTFVZERnUVdCQlRxSmpxSjlxa01aRTVodWM5amMyWDQKYXpoUUhEQU9CZ05WSFE4QkFmOEVCQU1DQnNBd0RBWURWUjBUQVFIL0JBSXdBRENDQWpzR0NTcUdTSWI0VFFFTgpBUVNDQWl3d2dnSW9NQjRHQ2lxR1NJYjRUUUVOQVFFRUVHWm5pVkZLTDZUOGJHVTNIWFBxMGNrd2dnRmxCZ29xCmhraUcrRTBCRFFFQ01JSUJWVEFRQmdzcWhraUcrRTBCRFFFQ0FRSUJEakFRQmdzcWhraUcrRTBCRFFFQ0FnSUIKRGpBUUJnc3Foa2lHK0UwQkRRRUNBd0lCQXpBUUJnc3Foa2lHK0UwQkRRRUNCQUlCQXpBUkJnc3Foa2lHK0UwQgpEUUVDQlFJQ0FQOHdFUVlMS29aSWh2aE5BUTBCQWdZQ0FnRC9NQkFHQ3lxR1NJYjRUUUVOQVFJSEFnRUJNQkFHCkN5cUdTSWI0VFFFTkFRSUlBZ0VBTUJBR0N5cUdTSWI0VFFFTkFRSUpBZ0VBTUJBR0N5cUdTSWI0VFFFTkFRSUsKQWdFQU1CQUdDeXFHU0liNFRRRU5BUUlMQWdFQU1CQUdDeXFHU0liNFRRRU5BUUlNQWdFQU1CQUdDeXFHU0liNApUUUVOQVFJTkFnRUFNQkFHQ3lxR1NJYjRUUUVOQVFJT0FnRUFNQkFHQ3lxR1NJYjRUUUVOQVFJUEFnRUFNQkFHCkN5cUdTSWI0VFFFTkFRSVFBZ0VBTUJBR0N5cUdTSWI0VFFFTkFRSVJBZ0VOTUI4R0N5cUdTSWI0VFFFTkFRSVMKQkJBT0RnTUQvLzhCQUFBQUFBQUFBQUFBTUJBR0NpcUdTSWI0VFFFTkFRTUVBZ0FBTUJRR0NpcUdTSWI0VFFFTgpBUVFFQmdCZ2FnQUFBREFQQmdvcWhraUcrRTBCRFFFRkNnRUJNQjRHQ2lxR1NJYjRUUUVOQVFZRUVIYldKVFVhCkNYdTYwOTJVMDI1MnFpRXdSQVlLS29aSWh2aE5BUTBCQnpBMk1CQUdDeXFHU0liNFRRRU5BUWNCQVFIL01CQUcKQ3lxR1NJYjRUUUVOQVFjQ0FRRUFNQkFHQ3lxR1NJYjRUUUVOQVFjREFRRUFNQW9HQ0NxR1NNNDlCQU1DQTBjQQpNRVFDSUVyZFJDZStZZ0Vmd09YeE10a3JHT3AvZHhIa1NodWRjaTNRSFlCYlNIK3ZBaUFRb291NmhtTHQwSHUvCnYyaGRFTGtOZmZJMGdxWjVwL2tkN0p1SzFOVHA4UT09Ci0tLS0tRU5EIENFUlRJRklDQVRFLS0tLS0KLS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSUNsakNDQWoyZ0F3SUJBZ0lWQUpWdlhjMjlHK0hwUUVuSjFQUXp6Z0ZYQzk1VU1Bb0dDQ3FHU000OUJBTUMKTUdneEdqQVlCZ05WQkFNTUVVbHVkR1ZzSUZOSFdDQlNiMjkwSUVOQk1Sb3dHQVlEVlFRS0RCRkpiblJsYkNCRApiM0p3YjNKaGRHbHZiakVVTUJJR0ExVUVCd3dMVTJGdWRHRWdRMnhoY21FeEN6QUpCZ05WQkFnTUFrTkJNUXN3CkNRWURWUVFHRXdKVlV6QWVGdzB4T0RBMU1qRXhNRFV3TVRCYUZ3MHpNekExTWpFeE1EVXdNVEJhTUhBeElqQWcKQmdOVkJBTU1HVWx1ZEdWc0lGTkhXQ0JRUTBzZ1VHeGhkR1p2Y20wZ1EwRXhHakFZQmdOVkJBb01FVWx1ZEdWcwpJRU52Y25CdmNtRjBhVzl1TVJRd0VnWURWUVFIREF0VFlXNTBZU0JEYkdGeVlURUxNQWtHQTFVRUNBd0NRMEV4CkN6QUpCZ05WQkFZVEFsVlRNRmt3RXdZSEtvWkl6ajBDQVFZSUtvWkl6ajBEQVFjRFFnQUVOU0IvN3QyMWxYU08KMkN1enB4dzc0ZUpCNzJFeURHZ1c1clhDdHgydFZUTHE2aEtrNnorVWlSWkNucVI3cHNPdmdxRmVTeGxtVGxKbAplVG1pMldZejNxT0J1ekNCdURBZkJnTlZIU01FR0RBV2dCUWlaUXpXV3AwMGlmT0R0SlZTdjFBYk9TY0dyREJTCkJnTlZIUjhFU3pCSk1FZWdSYUJEaGtGb2RIUndjem92TDJObGNuUnBabWxqWVhSbGN5NTBjblZ6ZEdWa2MyVnkKZG1salpYTXVhVzUwWld3dVkyOXRMMGx1ZEdWc1UwZFlVbTl2ZEVOQkxtUmxjakFkQmdOVkhRNEVGZ1FVbFc5ZAp6YjBiNGVsQVNjblU5RFBPQVZjTDNsUXdEZ1lEVlIwUEFRSC9CQVFEQWdFR01CSUdBMVVkRXdFQi93UUlNQVlCCkFmOENBUUF3Q2dZSUtvWkl6ajBFQXdJRFJ3QXdSQUlnWHNWa2kwdytpNlZZR1czVUYvMjJ1YVhlMFlKRGoxVWUKbkErVGpEMWFpNWNDSUNZYjFTQW1ENXhrZlRWcHZvNFVveWlTWXhyRFdMbVVSNENJOU5LeWZQTisKLS0tLS1FTkQgQ0VSVElGSUNBVEUtLS0tLQotLS0tLUJFR0lOIENFUlRJRklDQVRFLS0tLS0KTUlJQ2p6Q0NBalNnQXdJQkFnSVVJbVVNMWxxZE5JbnpnN1NWVXI5UUd6a25CcXd3Q2dZSUtvWkl6ajBFQXdJdwphREVhTUJnR0ExVUVBd3dSU1c1MFpXd2dVMGRZSUZKdmIzUWdRMEV4R2pBWUJnTlZCQW9NRVVsdWRHVnNJRU52CmNuQnZjbUYwYVc5dU1SUXdFZ1lEVlFRSERBdFRZVzUwWVNCRGJHRnlZVEVMTUFrR0ExVUVDQXdDUTBFeEN6QUoKQmdOVkJBWVRBbFZUTUI0WERURTRNRFV5TVRFd05EVXhNRm9YRFRRNU1USXpNVEl6TlRrMU9Wb3dhREVhTUJnRwpBMVVFQXd3UlNXNTBaV3dnVTBkWUlGSnZiM1FnUTBFeEdqQVlCZ05WQkFvTUVVbHVkR1ZzSUVOdmNuQnZjbUYwCmFXOXVNUlF3RWdZRFZRUUhEQXRUWVc1MFlTQkRiR0Z5WVRFTE1Ba0dBMVVFQ0F3Q1EwRXhDekFKQmdOVkJBWVQKQWxWVE1Ga3dFd1lIS29aSXpqMENBUVlJS29aSXpqMERBUWNEUWdBRUM2bkV3TURJWVpPai9pUFdzQ3phRUtpNwoxT2lPU0xSRmhXR2pibkJWSmZWbmtZNHUzSWprRFlZTDBNeE80bXFzeVlqbEJhbFRWWXhGUDJzSkJLNXpsS09CCnV6Q0J1REFmQmdOVkhTTUVHREFXZ0JRaVpReldXcDAwaWZPRHRKVlN2MUFiT1NjR3JEQlNCZ05WSFI4RVN6QkoKTUVlZ1JhQkRoa0ZvZEhSd2N6b3ZMMk5sY25ScFptbGpZWFJsY3k1MGNuVnpkR1ZrYzJWeWRtbGpaWE11YVc1MApaV3d1WTI5dEwwbHVkR1ZzVTBkWVVtOXZkRU5CTG1SbGNqQWRCZ05WSFE0RUZnUVVJbVVNMWxxZE5JbnpnN1NWClVyOVFHemtuQnF3d0RnWURWUjBQQVFIL0JBUURBZ0VHTUJJR0ExVWRFd0VCL3dRSU1BWUJBZjhDQVFFd0NnWUkKS29aSXpqMEVBd0lEU1FBd1JnSWhBT1cvNVFrUitTOUNpU0RjTm9vd0x1UFJMc1dHZi9ZaTdHU1g5NEJnd1R3ZwpBaUVBNEowbHJIb01zK1hvNW8vc1g2TzlRV3hIUkF2WlVHT2RSUTdjdnFSWGFxST0KLS0tLS1FTkQgQ0VSVElGSUNBVEUtLS0tLQoAMA0GCSqGSIb3DQEBCwUAA4IBAQDsI6gUzJaf0NeSpf5kI+jOIIu9Yno/wW9wYMNw9Sgax27l02+pFnoZszz3zZ3r9gXHCf0SO4V67+S/x1EbHCZHrQE0A+HXg2vx2ZRg33hI3Ek+PyCISZIhIOZhXeloVhsrrMtaaDE/VNYU5zs9w5kZo0TNs0sOXMScvD0EL4FtswwCxegwH7UE/M6jjtFY5MxamObFg0jxNm953Kdo2s7Y/lX+adYncRng03+E3jYw2d/PbGjKvF6FrG8uDlP8ev9ywYAN7jNPfCYAm5nzRxUzeTmxHHVJyejftrMkm5t8GHjB7c99qk+cisSunhRPOXtwslUphRsugqmuJWksL5E/"],
        "n": "8k4qyJO-joCyZba59XchaoUEZ3AUYq6dfx_9CM63Tm7NxNE2h1HTIgvN8vzS9EkHCFNgh64fOPKTGKVLZdiloiEv9nNN75Rc7cyB65C2VLIbirE3A3mij3A3w1nq0VQnvICQi2EyEu0SRe9x8SLjx3pFAsOAIspEUqizTKZ23pAfen9hE1TB9B8KHTmYiCxA0ZA8A7ibi3q53OS8pp5gO6wRLKwLHu0tt2OXpMm4iJeuAkspt8NkRR0EtxvfDGXGDhyZggDWNL3FDfLVCldciwEajSFzo3s60D0nkscOoTQWturBbCNdgv1nWog_NfM3k7Q2ngyyW_l0NfQ9g1Qoqw",
        "e": "AQAB",
        "kid": "Ij6AamWiPzLXlbB6WkVTh0YDzOXjrugzBBXwIsS5MUQ="
    },
    {
        "alg": "RS256",
        "kty": "RSA",
        "use": "sig",
        "x5c": ["MIIDRDCCAiygAwIBAgIBATANBgkqhkiG9w0BAQsFADAzMTEwLwYDVQQDDChodHRwczovL2F0dGVzdGF0aW9uLnN3bi5hdHRlc3QuYXp1cmUubmV0MB4XDTI0MTExMjAzNTIzOVoXDTI1MTExMjAzNTIzOVowMzExMC8GA1UEAwwoaHR0cHM6Ly9hdHRlc3RhdGlvbi5zd24uYXR0ZXN0LmF6dXJlLm5ldDCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAK329mvReYEvJeFfxKrCDQBOS+10gNjOkuNQr75wmb7usqMfWymrEjuPtsy8Hmniu0koiZ8PT5xoX6liHHUB+yBHOP0ZQLjZrlkQbHT1vEQhs5aiU76Kd0N+jdJEWCM0jb6Dy45bgXG6O92o0c58ODMzMAfonlchKtZxqM1fdixEQceSb6PIXOymZJPF2q8SdXiKNc/p5eaN6QzldZo4bqOS0tToya34FA1IepwwA6okL+Pu0R9BgTlQnE1Rs8xMaXXS8lszVGalu14oX2yfKxHkV9T+0Fl0+MWKvzzhI8yb+AhhuC32cu4TAmx/ABMhgpTPGNLX0L74b+SsvO8lMw0CAwEAAaNjMGEwDwYDVR0TBAgwBgEB/wIBADAdBgNVHQ4EFgQUFQrRSmPARse8R1Zfpb3A42CvwKowHwYDVR0jBBgwFoAUFQrRSmPARse8R1Zfpb3A42CvwKowDgYDVR0PAQH/BAQDAgIEMA0GCSqGSIb3DQEBCwUAA4IBAQBmw6jROda8zAh5U29ih1fDT/lz/u8pJpI/HW2a6pUBunxiuBM9uKmbaKLQ6InAeVrA3uhbK0TgJ/o2vXdOf9Dk90zClIf4jYyyPmVbRv76QBZOaPPOQALqil9BUGwyT4hCfB+obn9llTBcGwPieX8FNgmlEQqwUphttKnRB1LX2LlzrIsLVzVtCDnMEYc8MEQkROw/RWOTSQm1S/x0+2v4GHIDy4jwRokrxpqtKwZW/FmvlYDmN3KqJbcJhtNlyrA9g7gJr3XzhTVK0K5B7Fwf2+2XmzxGzMvcs5zbdaPLQQPc5GP9rgExZ0y/gzrVinrFy+777yqhY96lUy8agg0O"],
        "n": "rfb2a9F5gS8l4V_EqsINAE5L7XSA2M6S41CvvnCZvu6yox9bKasSO4-2zLweaeK7SSiJnw9PnGhfqWIcdQH7IEc4_RlAuNmuWRBsdPW8RCGzlqJTvop3Q36N0kRYIzSNvoPLjluBcbo73ajRznw4MzMwB-ieVyEq1nGozV92LERBx5Jvo8hc7KZkk8XarxJ1eIo1z-nl5o3pDOV1mjhuo5LS1OjJrfgUDUh6nDADqiQv4-7RH0GBOVCcTVGzzExpddLyWzNUZqW7XihfbJ8rEeRX1P7QWXT4xYq_POEjzJv4CGG4LfZy7hMCbH8AEyGClM8Y0tfQvvhv5Ky87yUzDQ",
        "e": "AQAB",
        "kid": "2ZY15k4GubQk5fAFMNPBhGtBUr04/BW7m/g94PDprI0="
    }]
}"###;
    fn get_test_cert() -> MaaCert {
        serde_json::from_str(TEST_MAA_CERT).unwrap()
    }
    #[tokio::test]
    async fn maa_get_signing_certs_returns_at_least_one_key() -> AttestationResult<()> {
        let client = MaaClient::new("https://attestation.swn.attest.azure.net", "2022-08-01 ");
        let res = client.maa_get_signing_certs().await?;
        assert!(res.keys.len() > 0);
        Ok(())
    }

    #[test]
    fn correct_cert_response_deserializes() -> AttestationResult<()> {
        let _cert = get_test_cert();
        Ok(())
    }


    #[tokio::test]
    async fn incorrect_url_throws_err() -> AttestationResult<()> {
        let result = MaaClient::new("www.google.com", "api-1").maa_get_signing_certs().await;
        assert!(result.is_err());
        Ok(())
    }
}
