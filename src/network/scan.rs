use futures::future::join_all;
use if_addrs::{get_if_addrs, IfAddr, Ifv4Addr, Ifv6Addr, Interface};
use ipnetwork::{IpNetworkError, Ipv4Network};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{Ipv4Addr, TcpListener};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use super::utils::{get_interfaces, octets_to_prefix};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IpPortPair {
    pub ip: Ipv4Addr,
    pub port: u16,
}

/// Find an available port
/// ```
/// use tauri_plugin_network::network::scan::find_available_port;
/// let port = find_available_port().unwrap();
/// assert!(port > 0);
/// ```
pub fn find_available_port() -> Result<u16, std::io::Error> {
    // Try binding to port 0 to let the OS choose an available port.
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let local_addr = listener.local_addr()?;
    Ok(local_addr.port())
}

/// check if a port is occupied
/// ```
/// use tauri_plugin_network::network::scan::{is_port_taken};
/// let taken = is_port_taken(9876);
/// assert!(!taken);
/// ```
pub fn is_port_taken(port: u16) -> bool {
    TcpListener::bind(String::from("127.0.0.1:") + &port.to_string()).is_err()
}

/// use multiple threads to find an available port from a list of candidate ports
/// ```
/// use tauri_plugin_network::network::scan::{find_available_port_from_list};
/// let candidate_ports = vec![80, 8081, 8082, 8083, 8084];
/// if let Some(port) = find_available_port_from_list(candidate_ports) {
///     println!("port {} is available", port);
///     assert!(port > 0);
/// } else {
///     println!("no available port");
/// }
/// ```
pub fn find_available_port_from_list(candidate_ports: Vec<u16>) -> Option<u16> {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    for port in candidate_ports {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let is_port_taken = is_port_taken(port);
            tx.send((port, is_port_taken)).unwrap();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    drop(tx);
    for (port, is_port_taken) in rx {
        if !is_port_taken {
            return Some(port);
        }
    }
    None
}

/// Verify is a port is open by checking the keyword. Check if returned text match exactly with the keyword
/// ```ignore
/// use tauri_plugin_network::network::scan::is_http_port_open_by_keyword;
/// let online = is_http_port_open("localhost".to_string(), 8000, None).await;
/// let online = is_http_port_open("localhost".to_string(), 8000, Some("CrossCopy".to_string())).await;
/// assert!(!online);
/// ```
pub async fn is_http_port_open(ip: String, port: u16, keyword: Option<String>) -> bool {
    let scan_addr: String = format!("{}:{}", ip, port);
    let url = format!("http://{}", scan_addr);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    // let result = client.get(&url).send().await?;
    // let response_text = result.text().await?;
    // let is_open = match keyword {
    //     Some(keyword) => response_text == keyword,
    //     None => true,
    // };
    // Ok(false)
    match client.get(&url).send().await {
        Ok(result) => match result.text().await {
            Ok(text) => match keyword {
                Some(keyword) => text == keyword,
                None => true,
            },
            Err(_) => false,
        },
        Err(_) => false,
    }
}

/// Scan for online ips with given ip port pairs. If keyword is specified, check returned text; otherwise, only check if port is open
/// ```ignore
/// let ip = Ipv4Addr::new(127, 0, 0, 1);
/// let online_ips = scan_online_ip_port_pairs(
///     vec![(ip, 8000), (Ipv4Addr::new(192, 168, 1, 13), 9000)],
///     None,
/// ).await;
/// ```
pub async fn scan_online_ip_port_pairs(
    ip_port_pairs: &[IpPortPair],
    keyword: Option<String>,
) -> Vec<IpPortPair> {
    let mut ret: Vec<IpPortPair> = Vec::new();
    let mut handles = Vec::new();
    for port_pair in ip_port_pairs.iter().copied() {
        let c = tokio::spawn({
            let keyword3 = keyword.clone();
            async move {
                is_http_port_open(port_pair.ip.clone().to_string(), port_pair.port, keyword3).await
            }
        });
        handles.push(c);
    }
    let results = join_all(handles).await;
    let results_list: Vec<_> = results.into_iter().map(|res| res.unwrap()).collect();
    for (i, online) in results_list.iter().enumerate() {
        if *online {
            ret.push(ip_port_pairs[i]);
        }
    }
    ret
}

/// Scan online ips with optional keyword and a fixed port. If keyword is specified, check returned text; otherwise, only check if port is open
pub async fn scan_online_ips(
    ips: Vec<Ipv4Addr>,
    port: u16,
    keyword: Option<String>,
) -> Vec<Ipv4Addr> {
    // construct ip port pairs
    let ip_port_pairs: Vec<IpPortPair> = ips
        .to_vec()
        .iter()
        .map(|ip| IpPortPair { ip: *ip, port })
        .collect();
    scan_online_ip_port_pairs(&ip_port_pairs, keyword)
        .await
        .into_iter()
        .map(|pair| pair.ip)
        .collect()
}

pub fn interface_to_network(ifv4_addr: Ifv4Addr) -> Result<Ipv4Network, IpNetworkError> {
    let prefix = octets_to_prefix(ifv4_addr.netmask.octets());
    let network = Ipv4Network::new(ifv4_addr.ip, prefix)?;
    Ok(network)
}

pub fn ipv4_network_to_ips(network: Ipv4Network) -> Vec<Ipv4Addr> {
    network.iter().collect::<Vec<Ipv4Addr>>()
}

pub fn get_network_interfaces() -> std::io::Result<Vec<if_addrs::Interface>> {
    get_if_addrs()
}

pub fn is_ipv4_interface(if_addr: &if_addrs::Interface) -> bool {
    is_ipv4_addr(&if_addr.addr)
}

pub fn is_ipv4_addr(addr: &IfAddr) -> bool {
    match addr {
        IfAddr::V4(_) => true,
        IfAddr::V6(_) => false,
    }
}

/// IfAddr can be either IfAddr::V4 ir IfAddr::V6, we return None if it's V6 and Some(V4) if it's V4
pub fn get_ipv4_addr(addr: IfAddr) -> Option<Ifv4Addr> {
    match addr {
        IfAddr::V4(ifv4_addr) => Some(ifv4_addr),
        IfAddr::V6(_) => None,
    }
}

pub fn get_ipv6_addr(addr: IfAddr) -> Option<Ifv6Addr> {
    match addr {
        IfAddr::V6(ifv6_addr) => Some(ifv6_addr),
        IfAddr::V4(_) => None,
    }
}

pub fn get_ipv4_interfaces() -> Vec<if_addrs::Interface> {
    if let Ok(if_addrs) = get_network_interfaces() {
        if_addrs
            .iter()
            .filter(|if_addr| is_ipv4_interface(if_addr))
            .cloned()
            .collect()
    } else {
        Vec::new()
    }
}

/// Returns a HashMap of interface names and their corresponding Ipv4Network
pub fn get_ipv4_interface_networks_map() -> HashMap<String, Ipv4Network> {
    let ipv4_interfaces = get_ipv4_interfaces();
    let mut networks = HashMap::new();
    for if_addr in ipv4_interfaces {
        if let Some(ifv4_addr) = get_ipv4_addr(if_addr.addr) {
            let network = interface_to_network(ifv4_addr).unwrap();
            networks.insert(if_addr.name, network);
        }
    }
    networks
}

/// ```
/// use tauri_plugin_network::network::scan::{get_ipv4_interface_networks};
/// let networks = get_ipv4_interface_networks();
/// for network in networks {
///     println!("Network: {}", network);
///     println!("\tNetwork Mask: {}", network.mask());
///     println!("\tNetwork IP: {}", network.ip());
///     println!("\tPrefix: {}", network.prefix());
/// }
/// ```
pub fn get_ipv4_interface_networks() -> Vec<Ipv4Network> {
    let networks_map = get_ipv4_interface_networks_map();
    networks_map.into_values().collect()
}

pub fn is_localhost(ip: &Ipv4Addr) -> bool {
    ip.is_loopback()
}

pub fn is_loopback_interface(interface: &Interface) -> bool {
    interface.is_loopback()
}

pub fn is_loopback_network(network: &Ipv4Network) -> bool {
    network.ip().is_loopback()
}

pub fn filter_out_loopback_interfaces(interfaces: Vec<Interface>) -> Vec<Interface> {
    interfaces
        .into_iter()
        .filter(|interface| !interface.is_loopback())
        .collect()
}

pub fn non_localhost_networks() -> Result<Vec<Ipv4Network>, network_interface::Error> {
    let interfaces = get_interfaces()?;
    let mut networks = Vec::new();
    for iface in interfaces {
        if !iface.v4_addrs.is_empty() {
            let v4_addrs = iface.v4_addrs;
            for ifaddr in v4_addrs {
                if let Some(net) = ifaddr.network {
                    if !is_loopback_network(&net) {
                        networks.push(net);
                    }
                }
            }
        }
    }
    Ok(networks)
}

pub async fn scan_local_network_online_hosts_by_port(
    port: u16,
    keyword: Option<String>,
) -> Result<Vec<IpPortPair>, network_interface::Error> {
    let networks = non_localhost_networks()?;
    let mut ip_port_pairs_to_scan: Vec<IpPortPair> = Vec::new();

    for network in networks {
        let ips = ipv4_network_to_ips(network);
        for ip in ips {
            ip_port_pairs_to_scan.push(IpPortPair { ip, port });
        }
    }
    let online_ip_port_pairs = scan_online_ip_port_pairs(&ip_port_pairs_to_scan, keyword).await;
    Ok(online_ip_port_pairs)
}

pub async fn local_server_is_running(port: u16, keyword: Option<String>) -> bool {
    is_http_port_open("localhost".to_string(), port, keyword).await
}

/// ```
/// use tauri_plugin_network::network::scan::{get_ipv4_interface_networks, filter_out_loopback_networks};
/// let networks = get_ipv4_interface_networks();
/// let networks = filter_out_loopback_networks(networks);
/// ```
pub fn filter_out_loopback_networks(networks: Vec<Ipv4Network>) -> Vec<Ipv4Network> {
    networks
        .into_iter()
        .filter(|network| !is_loopback_network(network))
        .collect()
}
