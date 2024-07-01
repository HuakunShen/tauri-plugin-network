use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
mod commands;
pub mod error;
pub mod model;
pub mod network;

pub use error::{Error, Result};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("network")
        .invoke_handler(tauri::generate_handler![
            commands::interface::get_interfaces,
            commands::interface::get_non_empty_interfaces,
            commands::scan::find_available_port,
            commands::scan::is_port_taken,
            commands::scan::is_http_port_open,
            commands::scan::scan_online_ip_port_pairs,
            commands::scan::scan_online_ips_by_port,
            commands::scan::non_localhost_networks,
            commands::scan::local_server_is_running,
            commands::scan::scan_local_network_online_hosts_by_port,
        ])
        .build()
}
