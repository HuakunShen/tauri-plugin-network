use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub mod commands;
pub mod common;
pub mod model;
pub mod network;
use common::MyState;

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
        ])
        .setup(|app| {
            app.manage(MyState::default());
            Ok(())
        })
        .build()
}
