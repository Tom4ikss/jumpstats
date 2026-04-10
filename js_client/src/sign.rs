include!(concat!(env!("OUT_DIR"), "/generated_secret.rs"));
use sha2::Sha256;
use hmac::{Hmac, Mac};
use chrono::Utc;

type HmacSha256 = Hmac<Sha256>;

pub fn sign_request(payload_json: &str) -> (String, String) {

    let secret = get_secret();

    let timestamp = Utc::now().timestamp().to_string();

    let data_to_sign = format!("{}{}", timestamp, payload_json);

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");

    mac.update(data_to_sign.as_bytes());

    let result = mac.finalize();
    let signature = hex::encode(result.into_bytes());

    (timestamp, signature)
}