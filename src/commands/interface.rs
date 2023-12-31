use crate::{model::interface::NetworkInterface, network};

#[tauri::command]
pub fn get_interfaces() -> Result<Vec<NetworkInterface>, String> {
    network::utils::get_interfaces().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_non_empty_interfaces() -> Result<Vec<NetworkInterface>, String> {
    network::utils::get_non_empty_interfaces().map_err(|e| e.to_string())
}
