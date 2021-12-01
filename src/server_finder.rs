use trust_dns_client::udp::UdpClientConnection;
use trust_dns_client::client::Client;
use trust_dns_client::rr::{DNSClass, RecordType, Name};
use std::str::FromStr;
use trust_dns_client::op::DnsResponse;
use trust_dns_client::proto::rr::RData;

pub struct ServerFinder;


impl ServerFinder {
    fn get_server(paymail_server: &str) -> Option<String> {
        let name = Name::from_str(&*format!("{}", paymail_server)).unwrap();

        let address = "8.8.8.8:53".parse().unwrap();
        let conn = UdpClientConnection::new(address).unwrap();

        // Fetch SRV using DNSSEC client and return.
        let dnssec_client = trust_dns_client::client::SyncClient::new(conn);

        return dnssec_client
                .query(&name, DNSClass::IN, RecordType::SRV)
                .unwrap()
                .answers()
                .get(0)
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handcash_cloudserverresult_succeeds() {
        let value = ServerFinder::get_server("handcash.io");
        assert_eq!(value, Some("cloud.handcash.io".to_string()));
    }
}
