use crate::common;
use crate::network;
use network_interface::{NetworkInterface, V4IfAddr};
use serde::{ser::Serializer, Serialize};
use tauri::{command, AppHandle, Runtime, State, Window};

#[tauri::command]
pub async fn get_interfaces<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<Vec<NetworkInterface>, String> {
    network::utils::get_interfaces().map_err(|e| e.to_string())
}
