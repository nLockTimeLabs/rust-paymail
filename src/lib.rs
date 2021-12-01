use std::collections::HashMap;

use crate::capabilities_finder::{CapabilitiesFinder, CapabilityError};
use serde_derive::Deserialize;
use std::time::Duration;

mod capabilities_finder;

pub const PAYMAIL_CONN_TIMEOUT: Duration = Duration::from_secs(10);

macro_rules! unwrap_option_or_return_err {
    ( $data:expr, $err:expr ) => {
        match $data {
            Some(x) => x,
            None => return Err($err),
        }
    };
}

macro_rules! unwrap_result_or_return_err {
    ( $data:expr, $err:expr ) => {
        match $data {
            Ok(x) => x,
            Err(e) => return Err($err),
        }
    };
}

#[derive(Debug)]
pub enum PaymailError {
    InvalidPaymailAddress,
    UnableToReachPaymailServer,
    InvalidPaymailServerResponse,
    CapabilitiesError(CapabilityError),
}

#[derive(Debug, Deserialize)]
pub struct VerifyPublicKeyOwnerResponse {
    handle: serde_json::Value,
    pubkey: serde_json::Value,

    #[serde(rename(deserialize = "match"))]
    _match: serde_json::Value,
}

pub fn public_key_belongs_to_paymail(
    public_key: &str,
    paymail: &str,
) -> Result<bool, PaymailError> {
    let mut splitter = paymail.split("@");
    let alias = unwrap_option_or_return_err!(splitter.next(), PaymailError::InvalidPaymailAddress);
    let paymail_server =
        unwrap_option_or_return_err!(splitter.next(), PaymailError::InvalidPaymailAddress);

    let templateUrl = CapabilitiesFinder::get_from_domain(paymail_server)
        .map_err(|capability_err| PaymailError::CapabilitiesError(capability_err))?
        .get_verifyPublicKeyOwnership_template(alias, paymail_server, public_key)
        .map_err(|capability_err| PaymailError::CapabilitiesError(capability_err))?;

    let json_resp = ureq::get(&*templateUrl)
        .timeout(PAYMAIL_CONN_TIMEOUT)
        .call()
        .map_err(|err| return PaymailError::UnableToReachPaymailServer)
        .map(|resp| {
            resp.into_json::<VerifyPublicKeyOwnerResponse>()
                .map_err(|_| PaymailError::InvalidPaymailServerResponse)
        })??;

    return json_resp
        ._match
        .as_bool()
        .ok_or(PaymailError::InvalidPaymailServerResponse);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joes_paymailAndPubkey_succeeds() {
        let value = public_key_belongs_to_paymail(
            "02ab2bf59040f03ebf68ec4629f22b59840c9701286018ec6e36938aec3cfc2f99",
            "joethomas@moneybutton.com",
        )
        .unwrap();

        assert_eq!(value, true);
    }
}
