use std::net::Ipv4Addr;

use crate::network::{self, scan::IpPortPair};

#[tauri::command]
pub fn find_available_port() -> Result<u16, String> {
    network::scan::find_available_port().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn is_port_taken(port: u16) -> Result<bool, String> {
    let taken = network::scan::is_port_taken(port);
    // println!("taken: {taken}");
    Ok(taken)
}

#[tauri::command]
pub async fn is_http_port_open(
    ip: String,
    port: u16,
    keyword: Option<String>,
) -> Result<bool, String> {
    Ok(network::scan::is_http_port_open(ip, port, keyword).await)
}

#[tauri::command]
pub async fn scan_online_ip_port_pairs(
    ip_port_pairs: Vec<IpPortPair>,
    keyword: Option<String>,
) -> Result<Vec<IpPortPair>, String> {
    println!("ip_port_pairs: {:?}", ip_port_pairs);
    Ok(network::scan::scan_online_ip_port_pairs(&ip_port_pairs, keyword).await)
}

#[tauri::command]
pub async fn scan_online_ips_by_port(
    ips: Vec<Ipv4Addr>,
    port: u16,
    keyword: Option<String>,
) -> Result<Vec<Ipv4Addr>, String> {
    println!("IPs: {:?}", ips);
    println!("Keyword: {:?}", keyword);
    Ok(network::scan::scan_online_ips(ips, port, keyword).await)
}
