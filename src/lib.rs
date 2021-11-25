use crate::brfc::BRFC;
use std::collections::HashMap;
use serde_json::Value;

mod brfc;

pub const DOMAIN_PLACEHOLDER: &'static str = "{domain.tld}";
pub const ALIAS_PLACEHOLDER: &'static str = "{alias}";
pub const PUBKEY_PLACEHOLDER: &'static str = "{pubkey}";

macro_rules! unwrap_option_or_return_err {
    ( $data:expr, $err:expr ) => {
        match $data {
            Some(x) => x,
            None => return Err($err),
        }
    }
}

macro_rules! unwrap_result_or_return_err {
    ( $data:expr, $err:expr ) => {
        match $data {
            Ok(x) => x,
            Err(e) => return Err($err),
        }
    }
}

pub enum PaymailError {
    InvalidPaymailAddress,
    CapabilityUnavailable,
    UnableToReachPaymailServer
}

pub fn public_key_belongs_to_paymail(public_key: Vec<u8>, paymail: String) -> Result<bool, PaymailError> {
    let mut splitter = paymail.split("@");
    let username = unwrap_option_or_return_err!(splitter.next(), PaymailError::InvalidPaymailAddress);
    let paymail_server = unwrap_option_or_return_err!(splitter.next(), PaymailError::InvalidPaymailAddress);

    let capabilities = get_capabilities(paymail_server);
    let template = match capabilities.get(VERIFY_PUBLIC_KEY_OWNERSHIP.get_id()) {
        Some(template) => template,
        None => return Err(PaymailError::CapabilityUnavailable)
    };

    let verification_url = template
        .replace(DOMAIN_PLACEHOLDER, paymail_server)
        .replace(ALIAS_PLACEHOLDER, paymail_server)
        .replace(PUBKEY_PLACEHOLDER, &*hex::encode(public_key));

    let resp = unwrap_result_or_return_err!(
        http::handle().get(verification_url).exec(),
        PaymailError::UnableToReachPaymailServer
    );

    if resp.get_code() != 200 {
        println!("Unable to handle HTTP response code {}", resp.get_code());
    }

    let body = std::str::from_utf8(resp.get_body()).unwrap_or_else(|e| {
        panic!("Failed to parse response from {}; error is {}", url, e);
    });

    let json: Value = serde_json::from_str(body).unwrap_or_else(|e| {
        panic!("Failed to parse json; error is {}", e);
    });

    return Ok(false);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
