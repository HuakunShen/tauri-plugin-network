import { invoke } from "@tauri-apps/api/core"
import { Ipv4Network } from "./models"
import { HttpScanOptions, IpPortPair, NetworkInterface } from "./types"

export * from "./types"

export function getInterfaces(): Promise<Array<NetworkInterface>> {
  return invoke("plugin:network|get_interfaces")
}

export function getNonEmptyInterfaces(): Promise<Array<NetworkInterface>> {
  return invoke("plugin:network|get_non_empty_interfaces")
}

export function findAvailablePort(): Promise<number> {
  return invoke("plugin:network|find_available_port")
}
/**
 * This command doesn't work yet
 * @param port
 * @returns
 */
export function isPortTaken(port: number): Promise<boolean> {
  return invoke("plugin:network|is_port_taken", { port })
}

/**
 * Check if an http service is on given ip and port, and optional keyword, route, protocol
 * By default, it will send a get request to `http://<ip>:<port>` to see it a response is received.
 * If route is provided, it will send a get request to `http://<ip>:<port>/<route>`
 * If keyword is provided, it will check if the response contains the keyword (case insensitive)
 * @param ip
 * @param options
 * @returns
 */
export function isHttpPortOpen(ip: string, options?: HttpScanOptions): Promise<boolean> {
  return invoke("plugin:network|is_http_port_open", { ip, ...options })
}

/**
 * Given a list of IP-Port pairs, check if the service is online, with optional keyword, route, protocol
 * ```ts
 * // A IP Port Pair has the following type
 * type IpPortPair = {
 *     ip: string;
 *     port: number;
 * }
 * ```
 * @param ipPortPairs
 * @param options
 * @returns IP-Port pairs that are online
 */
export function scanOnlineIpPortPairs(
  ipPortPairs: IpPortPair[],
  options?: Omit<HttpScanOptions, "port">,
): Promise<IpPortPair[]> {
  return invoke("plugin:network|scan_online_ip_port_pairs", {
    ipPortPairs,
    ...options,
  })
}

/**
 * Similar to `scanOnlineIpPortPairs`, but only for a single port
 * @param ips
 * @param options
 * @returns
 */
export function scanOnlineIpsByPort(ips: string[], options?: HttpScanOptions): Promise<string[]> {
  return invoke("plugin:network|scan_online_ips_by_port", {
    ips,
    ...options,
  })
}

/**
 * Find all non-localhost IPV4 networks in all network interfaces
 * The interface without any IPV4 address is ignored
 * @returns Ipv4Network
 */
export function nonLocalhostNetworks() {
  return invoke<Ipv4Network[]>("plugin:network|non_localhost_networks")
}

/**
 * Check if a localhost server is running on given port, with optional keyword, route, protocol
 * @param options
 * @returns
 */
export function localServerIsRunning(options?: HttpScanOptions): Promise<boolean> {
  return invoke("plugin:network|local_server_is_running", options)
}

/**
 * Scan entire local network (exclude localhost) with http/https on optional route and an optional keyword.
 * @param options
 * @returns
 */
export function scanLocalNetworkOnlineHostsByPort(options?: HttpScanOptions): Promise<IpPortPair[]> {
  return invoke("plugin:network|scan_local_network_online_hosts_by_port", options)
}
