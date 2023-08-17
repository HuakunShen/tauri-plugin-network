use crate::model::interface::NetworkInterface;
use network_interface as niface;
use network_interface::NetworkInterfaceConfig;

/// ```ignore
/// let prefix = octets_to_prefix([255, 255, 224, 0]);
/// assert_eq!(prefix, 19);
/// ```
pub fn octets_to_prefix(octets: [u8; 4]) -> u8 {
    let mut prefix_len = 0;
    // each octet is a u8 ranging from 0 to 255 (0x00 to 0xff)
    for octet in &octets {
        // 0x80 is 10000000 in binary
        let mut bit_mask = 0x80; // Start with the leftmost bit
        for _ in 0..8 {
            if octet & bit_mask != 0 {
                prefix_len += 1;
            } else {
                // when 8 bits of each byte match completely
                break; // Break if a zero is encountered
            }
            bit_mask >>= 1; // Move to the next bit
        }
    }
    prefix_len
}

pub fn get_interfaces() -> niface::Result<Vec<NetworkInterface>> {
    let ifaces = niface::NetworkInterface::show()?
        .iter()
        .map(|iface| iface.into())
        .collect();
    Ok(ifaces)
}

pub fn get_interface_by_name(name: &str) -> niface::Result<NetworkInterface> {
    let ifaces = get_interfaces()?;
    let iface = ifaces.into_iter().find(|iface| iface.name == name).unwrap();
    Ok(iface)
}

pub fn get_non_empty_interfaces() -> niface::Result<Vec<NetworkInterface>> {
    let ifaces = get_interfaces()?;
    let non_ifaces: Vec<NetworkInterface> = ifaces
        .into_iter()
        .filter(|iface| !iface.v4_addrs.is_empty())
        .collect();
    Ok(non_ifaces)
}

#[cfg(test)]
mod tests {
    use super::super::constant::IPV4_MAPPING;

    #[test]
    // generate tests with a bunch of common netmask values
    fn test_octets_to_prefix() {
        // iterate IPV4_MAPPING and test each value

        for (octets, prefix) in IPV4_MAPPING.iter() {
            let prefix2 = super::octets_to_prefix(*octets);
            assert_eq!(prefix, &prefix2);
        }
    }
}
