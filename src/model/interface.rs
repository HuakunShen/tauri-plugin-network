use ipnetwork::{ipv4_mask_to_prefix, ipv6_mask_to_prefix, Ipv4Network, Ipv6Network};
use network_interface as niface;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};

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
    pub network: Option<Ipv4Network>,
    // pub size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct V6IfAddr {
    pub ip: Ipv6Addr,
    pub ip_octets: [u8; 16],
    pub broadcast: Option<Ipv6Addr>,
    pub broadcast_octets: Option<[u8; 16]>,
    pub netmask: Option<Ipv6Addr>,
    pub netmask_octets: Option<[u8; 16]>,
    pub prefix: Option<u8>,
    pub network: Option<Ipv6Network>,
    // pub size: Option<u128>, // u128 is too large for JavaScript and will result in error
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub v4_addrs: Vec<V4IfAddr>,
    pub v6_addrs: Vec<V6IfAddr>,
    pub mac_addr: Option<String>,
    pub index: u32,
}

pub fn v4_iface_to_network(addr: &niface::V4IfAddr) -> Option<Ipv4Network> {
    match addr.netmask {
        Some(netmask) => match Ipv4Network::with_netmask(addr.ip, netmask) {
            Ok(network) => Some(network),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn v6_iface_to_network(addr: &niface::V6IfAddr) -> Option<Ipv6Network> {
    match addr.netmask {
        Some(netmask) => match Ipv6Network::with_netmask(addr.ip, netmask) {
            Ok(network) => Some(network),
            Err(_) => None,
        },
        None => None,
    }
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
                        let network = v4_iface_to_network(addr);
                        Some(V4IfAddr {
                            ip: addr.ip,
                            ip_octets: addr.ip.octets(),
                            broadcast: addr.broadcast,
                            broadcast_octets: addr.broadcast.map(|broadcast| broadcast.octets()),
                            netmask: addr.netmask,
                            netmask_octets: addr.netmask.map(|netmask| netmask.octets()),
                            prefix: match addr.netmask {
                                Some(netmask) => match ipv4_mask_to_prefix(netmask) {
                                    Ok(prefix) => Some(prefix),
                                    Err(_) => None,
                                },
                                None => None,
                            },
                            network,
                            // size: match network {
                            //     Some(net) => Some(net.size()),
                            //     None => None,
                            // },
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
                        let network = v6_iface_to_network(addr);
                        Some(V6IfAddr {
                            ip: addr.ip,
                            ip_octets: addr.ip.octets(),
                            broadcast: addr.broadcast,
                            broadcast_octets: addr.broadcast.map(|broadcast| broadcast.octets()),
                            netmask: addr.netmask,
                            netmask_octets: addr.netmask.map(|netmask| netmask.octets()),
                            prefix: match addr.netmask {
                                Some(netmask) => match ipv6_mask_to_prefix(netmask) {
                                    Ok(prefix) => Some(prefix),
                                    Err(_) => None,
                                },
                                None => None,
                            },
                            network,
                            // size: match network {
                            //     Some(net) => Some(net.size()),
                            //     None => None,
                            // },
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
