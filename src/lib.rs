use network_interface::NetworkInterface;
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, State, Window,
};

pub mod commands;
pub mod common;
pub mod network;
use common::{MyState, Result};

#[command]
async fn execute<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, MyState>,
) -> Result<String> {
    state.0.lock().unwrap().insert("key".into(), "value".into());
    Ok("success".to_string())
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("network")
        .invoke_handler(tauri::generate_handler![
            execute,
            commands::interface::get_interfaces,
        ])
        .setup(|app| {
            app.manage(MyState::default());
            Ok(())
        })
        .build()
}
