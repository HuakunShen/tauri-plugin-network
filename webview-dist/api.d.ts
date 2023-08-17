import { IpPortPair, NetworkInterface } from "./types";
export * from "./types";
export declare function getInterfaces(): Promise<Array<NetworkInterface>>;
export declare function getNonEmptyInterfaces(): Promise<Array<NetworkInterface>>;
export declare function findAvailablePort(): Promise<number>;
/**
 * This command doesn't work yet
 * @param port
 * @returns
 */
export declare function isPortTaken(port: number): Promise<boolean>;
export declare function isHttpPortOpen(ip: string, port: number, keyword?: string): Promise<boolean>;
export declare function scanOnlineIpPortPairs(ipPortPairs: IpPortPair[], keyword?: string): Promise<IpPortPair[]>;
export declare function scanOnlineIpsByPort(ips: string[], port: number, keyword?: string): Promise<string[]>;
export declare function nonLocalhostNetworks(): Promise<unknown>;
export declare function localServerIsRunning(port: number, keyword?: string): Promise<boolean>;
export declare function scanLocalNetworkOnlineHostsByPort(port: number, keyword?: string): Promise<IpPortPair[]>;
