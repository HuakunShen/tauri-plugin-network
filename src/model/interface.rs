use std::net::{Ipv4Addr, Ipv6Addr};
use network_interface as niface;
use serde::{Deserialize, Serialize};

use crate::network::utils::octets_to_prefix;

#[derive(Debug, Serialize, Deserialize)]
pub struct MacAddr(pub [u8; 6]);

#[derive(Debug, Serialize, Deserialize)]
pub struct V4IfAddr {
    pub ip: Ipv4Addr,
    pub ip_octets: [u8; 4],
    pub broadcast: Option<Ipv4Addr>,
    pub broadcast_octets: Option<[u8; 4]>,
    pub netmask: Option<Ipv4Addr>,
    pub netmask_octets: Option<[u8; 4]>,
    pub prefix: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct V6IfAddr {
    pub ip: Ipv6Addr,
    pub ip_octets: [u8; 16],
    pub broadcast: Option<Ipv6Addr>,
    pub broadcast_octets: Option<[u8; 16]>,
    pub netmask: Option<Ipv6Addr>,
    pub netmask_octets: Option<[u8; 16]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub v4_addrs: Vec<V4IfAddr>,
    pub v6_addrs: Vec<V6IfAddr>,
    pub mac_addr: Option<String>,
    pub index: u32,
}

impl From<&niface::NetworkInterface> for NetworkInterface {
    fn from(iface: &niface::NetworkInterface) -> Self {
        NetworkInterface {
            name: iface.name.clone(),
            v4_addrs: iface
                .addr
                .iter()
                .filter_map(|addr| {
                    if let niface::Addr::V4(addr) = addr {
                        Some(V4IfAddr {
                            ip: addr.ip,
                            ip_octets: addr.ip.octets(),
                            broadcast: addr.broadcast,
                            broadcast_octets: match addr.broadcast {
                                Some(broadcast) => Some(broadcast.octets()),
                                None => None,
                            },
                            netmask: addr.netmask,
                            netmask_octets: match addr.netmask {
                                Some(netmask) => Some(netmask.octets()),
                                None => None,
                            },
                            prefix: match addr.netmask {
                                Some(netmask) => Some(octets_to_prefix(netmask.octets())),
                                None => None,
                            },
                        })
                    } else {
                        None
                    }
                })
                .collect(),
            v6_addrs: iface
                .addr
                .iter()
                .filter_map(|addr| {
                    if let niface::Addr::V6(addr) = addr {
                        Some(V6IfAddr {
                            ip: addr.ip,
                            ip_octets: addr.ip.octets(),
                            broadcast: addr.broadcast,
                            broadcast_octets: match addr.broadcast {
                                Some(broadcast) => Some(broadcast.octets()),
                                None => None,
                            },
                            netmask: addr.netmask,
                            netmask_octets: match addr.netmask {
                                Some(netmask) => Some(netmask.octets()),
                                None => None,
                            },
                        })
                    } else {
                        None
                    }
                })
                .collect(),
            mac_addr: iface.mac_addr.clone(),
            index: iface.index,
        }
    }
}
