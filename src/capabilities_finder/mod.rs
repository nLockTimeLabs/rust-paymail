use crate::capabilities_finder::brfc::BRFC;
use serde_derive::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::iter::Map;
use std::time::Duration;
use ureq::{Error, Response};

mod brfc;

pub const DOMAIN_PLACEHOLDER: &'static str = "{domain.tld}";
pub const ALIAS_PLACEHOLDER: &'static str = "{alias}";
pub const PUBKEY_PLACEHOLDER: &'static str = "{pubkey}";

#[derive(Deserialize)]
pub struct CapabilityResponse {
    bsvalias: serde_json::Value,
    capabilities: serde_json::Value,
}

#[derive(Debug)]
pub enum CapabilityError {
    CapabilityUnavailable,
    PaymailServerUnreachable,
    BadPaymailServerResponse,
}

pub(crate) struct CapabilitiesFinder {
    pub capabilities: HashMap<String, String>,
}

impl CapabilitiesFinder {
    pub(crate) fn get_from_domain(paymail_server: &str) -> Result<Self, CapabilityError> {
        let capabilities_url = format!("https://{}/.well-known/bsvalias", paymail_server);
        let capabilities_resp = ureq::get(&*capabilities_url)
            .timeout(Duration::from_secs(10))
            .call();

        let capabilities_map = match capabilities_resp {
            Ok(response) => match response.into_json::<CapabilityResponse>() {
                Ok(json) => json.capabilities,
                Err(_) => return Err(CapabilityError::BadPaymailServerResponse),
            },
            Err(_) => return Err(CapabilityError::PaymailServerUnreachable),
        };

        let mut capabilities = HashMap::new();

        match capabilities_map.as_object() {
            Some(obj) => {
                for (brfc_id, template_url) in obj {
                    template_url.as_str().map(|value| {
                        capabilities.insert(brfc_id.clone(), String::from(value));
                    });
                }
            }
            None => {
                return Err(CapabilityError::BadPaymailServerResponse);
            }
        }

        return Ok(CapabilitiesFinder { capabilities });
    }

    pub(crate) fn get_verifyPublicKeyOwnership_template(
        &self,
        alias: &str,
        domain: &str,
        pubkey: &str,
    ) -> Result<String, CapabilityError> {
        let templateUrl = match self
            .capabilities
            .get(brfc::VERIFY_PUBLIC_KEY_OWNERSHIP.get_id())
        {
            Some(template) => template,
            None => {
                return Err(CapabilityError::CapabilityUnavailable);
            }
        };

        return Ok(templateUrl
            .replace(DOMAIN_PLACEHOLDER, domain)
            .replace(ALIAS_PLACEHOLDER, alias)
            .replace(PUBKEY_PLACEHOLDER, pubkey));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_with_moneybutton_server() {
        let capabilities = CapabilitiesFinder::get_from_domain("moneybutton.com").unwrap();
    }
}
