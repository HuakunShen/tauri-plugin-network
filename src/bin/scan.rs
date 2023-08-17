use ipnetwork::{Ipv4Network, Ipv6Network};
use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use tauri_plugin_network::{
    commands::interface,
    network::{
        scan::is_ipv4_interface,
        utils::{get_interfaces, get_non_empty_interfaces},
    },
};

fn main() {
    // let interfaces = get_interfaces().unwrap();
    // for iface in interfaces {

    //     if is_ipv4_interface(iface) {

    //     }
    // }
    let ifaces = network_interface::NetworkInterface::show().unwrap();
    for iface in ifaces {
        for addr in iface.addr {
            match addr {
                network_interface::Addr::V4(addr) => {
                    println!("{}: {}", iface.name, addr.ip);
                    let network =
                        Ipv4Network::with_netmask(addr.ip, addr.netmask.unwrap()).unwrap();
                    // ipnetwork::IpNetwork
                    // let network = ipnetwork::IpNetwork::with_netmask(ipnetwork::IpAddr::V4(addr.ip), iface.netmask);
                }
                network_interface::Addr::V6(addr) => {
                    println!("{}: {}", iface.name, addr.ip);
                    let network =
                        Ipv6Network::with_netmask(addr.ip, addr.netmask.unwrap()).unwrap();
                }
            }
        }
    }
    // let networks = interfaces.iter().map(|iface| ipnetwork::IpNetwork::with_netmask(iface., netmask)).collect::<Vec<_>>();
    // for iface in interfaces {
    //     println!("{:?}", iface.addr);
    // }
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
