const COMMANDS: &[&str] = &[
    "get_interfaces",
    "get_non_empty_interfaces",
    "find_available_port",
    "is_port_taken",
    "is_http_port_open",
    "scan_online_ip_port_pairs",
    "scan_online_ips_by_port",
    "non_localhost_networks",
    "local_server_is_running",
    "scan_local_network_online_hosts_by_port",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
