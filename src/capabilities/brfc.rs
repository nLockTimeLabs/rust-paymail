use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::Hash;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VERIFY_PUBLIC_KEY_OWNERSHIP: BRFC = BRFC::new(
        "bsvalias public key verify (Verify Public Key Owner)",
        "",
        ""
    );
}

#[derive(Hash)]
pub struct BRFC {
    id: String,
}

impl BRFC {
    pub fn new(title: &str, author: &str, version: &str) -> Self {
        let brfc_id = bitcoin_hashes::sha256d::Hash::hash(
            format!("{}{}{}", title.trim(), author.trim(), version.trim()).as_bytes(),
        )
        .to_hex()[0..12]
            .to_string();

        return Self { id: brfc_id };
    }

    pub fn get_id(&self) -> &str {
        return &self.id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pubkey_ownership() {
        assert_eq!(VERIFY_PUBLIC_KEY_OWNERSHIP.get_id(), "a9f510c16bde")
    }
    #[test]
    fn happy_case_test_vector_1() {
        let brfc = BRFC::new("BRFC Specifications", "andy (nChain)", "1");
        assert_eq!(brfc.get_id(), "57dd1f54fc67");
    }

    #[test]
    fn happy_case_test_vector_2() {
        let brfc = BRFC::new(
            "bsvalias Payment Addressing (PayTo Protocol Prefix)",
            "andy (nChain)",
            "1",
        );
        assert_eq!(brfc.get_id(), "74524c4d6274");
    }
}
