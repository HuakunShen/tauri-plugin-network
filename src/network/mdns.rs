use super::utils::get_non_empty_interfaces;
use gethostname::gethostname;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr};

pub struct MdnsService {
    daemon: ServiceDaemon,
    service_type: String,
}

impl MdnsService {
    /// Sample service_type: _crosscopy._tcp.local.
    /// Must follow this format: `_{service_name}._{protocol}.local.`
    /// `service_name`` has to be a single work, not dash, no underscore, no space
    pub fn new(service_name: &str) -> Result<Self, mdns_sd::Error> {
        let daemon = ServiceDaemon::new()?;
        Ok(Self {
            daemon,
            service_type: format!("_mdns-{}._tcp.local.", service_name),
        })
    }

    pub fn get_daemon(&self) -> &ServiceDaemon {
        &self.daemon
    }

    /// Get the default IP addresses of the host machine, docker, veth and br network interfaces are excluded
    pub fn get_default_ips_str() -> String {
        let my_net_ifaces = get_non_empty_interfaces().unwrap();
        let ipv4_addrs = my_net_ifaces
            .iter()
            .filter(|iface| {
                !iface.name.starts_with("br")
                    && !iface.name.starts_with("docker")
                    && !iface.name.starts_with("veth")
            })
            .flat_map(|iface| iface.v4_addrs.iter())
            .collect::<Vec<_>>();
        let ipv4_ips: Vec<String> = ipv4_addrs
            .iter()
            .map(|ip| ip.to_owned().to_owned().ip.to_string())
            .collect::<Vec<_>>();
        let ipv4_ips_str = ipv4_ips.join(",");
        ipv4_ips_str
    }

    /// ```ignore
    /// mdns.register(
    ///     "test-instance",
    ///     &MdnsService::get_default_ips_str(),
    ///     1566,
    ///     None,
    ///     None,
    /// )
    /// .unwrap();
    /// ```
    pub fn register(
        &self,
        instance_name: &str,
        ip: &str,
        port: u16,
        host_name: Option<String>,
        properties: Option<HashMap<String, String>>,
    ) -> Result<ServiceInfo, mdns_sd::Error> {
        let hostname = format!("{}.local.", gethostname().to_string_lossy().to_string());
        let my_service = match properties {
            Some(properties) => ServiceInfo::new(
                &self.service_type,
                instance_name,
                &host_name.unwrap_or_else(|| hostname),
                ip,
                port,
                properties,
            )?,
            None => {
                let default_properties: [(&str, &str); 0] = [];
                ServiceInfo::new(
                    &self.service_type,
                    instance_name,
                    &host_name.unwrap_or_else(|| hostname),
                    ip,
                    port,
                    &default_properties[..],
                )?
            }
        };
        self.daemon.register(my_service.clone())?;
        Ok(my_service)
    }

    pub fn unregister(
        &self,
        instance_name: &str,
    ) -> Result<mdns_sd::Receiver<mdns_sd::UnregisterStatus>, mdns_sd::Error> {
        let fullname = format!("{}.{}", instance_name, &self.service_type);
        let rx: mdns_sd::Receiver<mdns_sd::UnregisterStatus> = self.daemon.unregister(&fullname)?;
        Ok(rx)
    }

    pub fn browse_w_closure<F>(&self, mut callback: F) -> Result<(), mdns_sd::Error>
    where
        F: FnMut(ServiceEvent) + Send + 'static,
    {
        let receiver = self.daemon.browse(&self.service_type)?;

        std::thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                callback(event);
            }
        });
        Ok(())
    }

    pub fn browse(&self) -> Result<mdns_sd::Receiver<ServiceEvent>, mdns_sd::Error> {
        self.daemon.browse(&self.service_type)
    }

    pub fn shutdown(&self) -> Result<(), mdns_sd::Error> {
        self.daemon.shutdown()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInfoMod {
    pub addresses: Vec<IpAddr>,
    pub fullname: String, // <instance>.<service>.<domain>
    pub hostname: String,
    pub port: u16,
    pub service_type: String, // Returns the service type including the domain label. For example: "_my-service._udp.local.".
    pub sub_type: Option<String>, // Returns the service subtype including the domain label, if subtype has been defined. For example: "_printer._sub._http._tcp.local.".
}

impl From<ServiceInfo> for ServiceInfoMod {
    fn from(info: ServiceInfo) -> Self {
        Self {
            addresses: info.get_addresses().iter().cloned().collect(),
            fullname: info.get_fullname().to_string(),
            hostname: info.get_hostname().to_string(),
            port: info.get_port(),
            service_type: info.get_type().to_string(),
            sub_type: info.get_subtype().clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdns_service() {
        let mdns = MdnsService::new("crosscopy").unwrap();
        mdns.register(
            "test-instance",
            &MdnsService::get_default_ips_str(),
            1566,
            None,
            None,
        )
        .unwrap();
        mdns.browse_w_closure(|event| match event {
            ServiceEvent::ServiceResolved(info) => {
                println!("Service up: {:?}", info);
            }
            _ => {}
        })
        .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        mdns.shutdown().unwrap();
    }
}
