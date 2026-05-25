// passkey utils

use std::str::FromStr;


#[cfg(not(target_arch="wasm32"))]
pub fn build_webauthn_instance() -> webauthn_rs::Webauthn {
    // this will always be a valid url
    let url = unsafe {url::Url::from_str("https://jt-morris.com").unwrap_unchecked()};
    const RP_ID: &str = "jt-morris.com";
    webauthn_rs::WebauthnBuilder::new(&RP_ID, &url).unwrap().allow_subdomains(true).rp_name("JordanMorris").build().unwrap()
}

pub fn bytes_to_hex_cookie_safe(b: &[u8]) -> String {
     b.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(not(target_arch="wasm32"))]
pub fn serialize_passkey_auth_state_as_hex_encoded_binary(auth_state: &webauthn_rs::prelude::PasskeyAuthentication) -> String {
    // serialize to bytes
    let b = bitcode::serialize(auth_state).expect("Serialization failed. Should always succeed?");
    bytes_to_hex_cookie_safe(&b)
}