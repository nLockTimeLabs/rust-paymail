use std::collections::HashMap;

use crate::capabilities::CapabilitiesFactory;

mod capabilities;


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

    return Ok(false);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
