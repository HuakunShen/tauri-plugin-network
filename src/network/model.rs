use ipnetwork::Ipv4Network;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Ipv4NetworkSerialize {
    addr: String,
    prefix: u8,
}

impl From<Ipv4Network> for Ipv4NetworkSerialize {
    fn from(network: Ipv4Network) -> Self {
        Ipv4NetworkSerialize {
            addr: network.ip().to_string(),
            prefix: network.prefix(),
        }
    }
}