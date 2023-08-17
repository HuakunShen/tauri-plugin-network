use network_interface::NetworkInterface;
use tauri_plugin_network::network::utils::{get_interfaces, get_non_empty_interfaces};

fn main() {
    let interfaces = get_interfaces().unwrap();
    for iface in interfaces {
        println!("{:?}", iface.addr);
    }
    // let interfaces = get_non_empty_interfaces().unwrap();
    // println!("{:?}", interfaces);
    // for interface in interfaces {
    //     println!("Name: {}", interface.name);
    //     println!("\taddr: {:?}", interface.addr);
    //     println!("\tmac_addr: {:?}", interface.mac_addr);
    //     println!("\tindex: {}", interface.index);
    // }
    // let addr = std::net::Ipv4Addr::new(1, 1, 1, 1);
    // let str = addr.to_string();
    // let addr2 = "192.168.2.3".parse::<std::net::Ipv4Addr>().unwrap();

    // println!("{}", addr2);
}
