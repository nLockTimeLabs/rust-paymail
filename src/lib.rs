use std::collections::HashMap;

use crate::capabilities::{CapabilitiesFactory, CapabilityError};
use std::time::Duration;
use serde_derive::{Deserialize};

mod capabilities;

pub const PAYMAIL_CONN_TIMEOUT: Duration = Duration::from_secs(10);

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

#[derive(Debug)]
pub enum PaymailError {
    InvalidPaymailAddress,
    UnableToReachPaymailServer,
    InvalidPaymailServerResponse,
    CapabilitiesError(CapabilityError)
}

#[derive(Debug, Deserialize)]
pub struct VerifyPublicKeyOwnerResponse {
    handle: serde_json::Value,
    pubkey: serde_json::Value,

    #[serde(rename(deserialize = "match"))]
    _match: serde_json::Value
}


pub fn public_key_belongs_to_paymail(public_key: &str, paymail: &str) -> Result<bool, PaymailError> {
    let mut splitter = paymail.split("@");
    let alias = unwrap_option_or_return_err!(splitter.next(), PaymailError::InvalidPaymailAddress);
    let paymail_server = unwrap_option_or_return_err!(splitter.next(), PaymailError::InvalidPaymailAddress);

    let templateUrl = CapabilitiesFactory::get_from_domain(paymail_server)
        .map_err(|capability_err|  PaymailError::CapabilitiesError(capability_err))?
        .get_verifyPublicKeyOwnership_template(alias, paymail_server, public_key)
        .map_err(|capability_err|  PaymailError::CapabilitiesError(capability_err))?;

    let json_resp  = ureq::get(&*templateUrl)
        .timeout(PAYMAIL_CONN_TIMEOUT)
        .call()
        .map_err(|err| return PaymailError::UnableToReachPaymailServer)
        .map(|resp| resp.into_json::<VerifyPublicKeyOwnerResponse>()
            .map_err(|_| PaymailError::InvalidPaymailServerResponse)
        )??;

    return json_resp._match.as_bool()
        .ok_or(PaymailError::InvalidPaymailServerResponse);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joes_paymailAndPubkey_succeeds() {
        let value = public_key_belongs_to_paymail("039288e2ce3b2a2bff50a90bf739c8ba0f8be87ec18010b8a6d8e90da9d2750ee1", "joethomas@moneybutton.com")
            .unwrap();

        assert_eq!(value, false);
    }
}
