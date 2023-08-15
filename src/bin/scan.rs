use network_interface::NetworkInterface;
use tauri_plugin_network::network::utils::{get_interfaces, get_non_empty_interfaces};

fn main() {
    // let interfaces = get_interfaces().unwrap();
    let interfaces = get_non_empty_interfaces().unwrap();
    // println!("{:?}", interfaces);
    for interface in interfaces {
        println!("Name: {}", interface.name);
        println!("\taddr: {:?}", interface.addr);
        println!("\tmac_addr: {:?}", interface.mac_addr);
        println!("\tindex: {}", interface.index);
    }
}
