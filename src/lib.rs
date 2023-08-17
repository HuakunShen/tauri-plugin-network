use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub mod commands;
pub mod common;
pub mod network;
pub mod model;
use common::MyState;

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("network")
        .invoke_handler(tauri::generate_handler![
            commands::interface::get_interfaces,
        ])
        .setup(|app| {
            app.manage(MyState::default());
            Ok(())
        })
        .build()
}
