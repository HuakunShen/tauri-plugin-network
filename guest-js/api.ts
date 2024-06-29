import { invoke } from "@tauri-apps/api/core";
import { IpPortPair, NetworkInterface } from "./types";
import { Ipv4Network } from "./models";
export * from "./types";

export function getInterfaces(): Promise<Array<NetworkInterface>> {
  return invoke("plugin:network|get_interfaces");
}

export function getNonEmptyInterfaces(): Promise<Array<NetworkInterface>> {
  return invoke("plugin:network|get_non_empty_interfaces");
}

export function findAvailablePort(): Promise<number> {
  return invoke("plugin:network|find_available_port");
}
/**
 * This command doesn't work yet
 * @param port
 * @returns
 */
export function isPortTaken(port: number): Promise<boolean> {
  return invoke("plugin:network|is_port_taken", { port });
}

export function isHttpPortOpen(
  ip: string,
  port: number,
  keyword?: string
): Promise<boolean> {
  return invoke("plugin:network|is_http_port_open", { ip, port, keyword });
}

export function scanOnlineIpPortPairs(
  ipPortPairs: IpPortPair[],
  keyword?: string
): Promise<IpPortPair[]> {
  return invoke("plugin:network|scan_online_ip_port_pairs", {
    ipPortPairs,
    keyword,
  });
}

export function scanOnlineIpsByPort(
  ips: string[],
  port: number,
  keyword?: string
): Promise<string[]> {
  return invoke("plugin:network|scan_online_ips_by_port", {
    ips,
    port,
    keyword,
  });
}

/**
 * Find all non-localhost IPV4 networks in all network interfaces
 * The interface without any IPV4 address is ignored
 * @returns Ipv4Network
 */
export function nonLocalhostNetworks() {
  return invoke<Ipv4Network[]>("plugin:network|non_localhost_networks");
}

export function localServerIsRunning(
  port: number,
  keyword?: string
): Promise<boolean> {
  return invoke("plugin:network|local_server_is_running", { port, keyword });
}

/**
 * Scan entire local network with a keyword.
 * Will scan with HTTP request on port 80. If resposne contains keyword then it is considered online.
 * @param port 
 * @param keyword 
 * @returns 
 */
export function scanLocalNetworkOnlineHostsByPort(
  port: number,
  keyword?: string
): Promise<IpPortPair[]> {
  return invoke("plugin:network|scan_local_network_online_hosts_by_port", {
    port,
    keyword,
  });
}
