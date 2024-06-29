use crate::network::{self, model::Ipv4NetworkSerialize, scan::IpPortPair};
use std::net::Ipv4Addr;

#[tauri::command]
pub fn find_available_port() -> Result<u16, String> {
    network::scan::find_available_port().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn is_port_taken(port: u16) -> Result<bool, String> {
    Ok(network::scan::is_port_taken(port))
}

#[tauri::command]
pub async fn is_http_port_open(
    ip: String,
    port: u16,
    keyword: Option<String>,
    route: Option<String>,
    protocol: Option<String>,
    status_code: Option<u16>,
) -> Result<bool, String> {
    Ok(network::scan::is_http_port_open(ip, port, keyword, route, protocol, status_code).await)
}

#[tauri::command]
pub async fn scan_online_ip_port_pairs(
    ip_port_pairs: Vec<IpPortPair>,
    keyword: Option<String>,
    route: Option<String>,
    protocol: Option<String>,
    status_code: Option<u16>,
) -> Result<Vec<IpPortPair>, String> {
    Ok(network::scan::scan_online_ip_port_pairs(
        &ip_port_pairs,
        keyword,
        route,
        protocol,
        status_code,
    )
    .await)
}

#[tauri::command]
pub async fn scan_online_ips_by_port(
    ips: Vec<Ipv4Addr>,
    port: u16,
    keyword: Option<String>,
    route: Option<String>,
    protocol: Option<String>,
    status_code: Option<u16>,
) -> Result<Vec<Ipv4Addr>, String> {
    Ok(network::scan::scan_online_ips(ips, port, keyword, route, protocol, status_code).await)
}

#[tauri::command]
pub async fn non_localhost_networks() -> Result<Vec<Ipv4NetworkSerialize>, String> {
    network::scan::non_localhost_networks()
        .map_err(|err| err.to_string())
        .map(|networks| networks.into_iter().map(|network| network.into()).collect())
}

#[tauri::command]
pub async fn local_server_is_running(
    port: u16,
    keyword: Option<String>,
    route: Option<String>,
    protocol: Option<String>,
    status_code: Option<u16>,
) -> Result<bool, String> {
    Ok(network::scan::local_server_is_running(port, keyword, route, protocol, status_code).await)
}

#[tauri::command]
pub async fn scan_local_network_online_hosts_by_port(
    port: u16,
    keyword: Option<String>,
    route: Option<String>,
    protocol: Option<String>,
    status_code: Option<u16>,
) -> Result<Vec<IpPortPair>, String> {
    network::scan::scan_local_network_online_hosts_by_port(
        port,
        keyword,
        route,
        protocol,
        status_code,
    )
    .await
    .map_err(|err| err.to_string())
}
