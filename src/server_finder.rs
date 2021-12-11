use srv_rs::{Execution, SrvClient,};
use trust_dns_resolver::{ Resolver};
use srv_rs::resolver::{SrvResolver};
use trust_dns_resolver::proto::rr::rdata::SRV;

pub struct ServerFinder;


impl ServerFinder {
    pub fn get_server(paymail_server: &str) -> Option<String> {
        let (conf, mut opts) = trust_dns_resolver::system_conf::read_system_conf().unwrap();
        let resolver = Resolver::new(conf, opts).ok()?;
        let srv_lookup = resolver.srv_lookup(format!("_bsvalias._tcp.{}", paymail_server)).ok()?;
        let srv_records = srv_lookup.into_iter().collect::<Vec<SRV>>();
        let first_record = srv_records.get(0)?;

        return Some(first_record.target().to_string().trim_end_matches(".").to_string());
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
