import { z } from "zod";
export declare const V6IfAddr: z.ZodObject<{
    ip: z.ZodString;
    ip_octets: z.ZodArray<z.ZodNumber, "many">;
    broadcast: z.ZodNullable<z.ZodString>;
    broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    netmask: z.ZodNullable<z.ZodString>;
    netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    prefix: z.ZodNullable<z.ZodNumber>;
    network: z.ZodNullable<z.ZodString>;
}, "strip", z.ZodTypeAny, {
    ip: string;
    ip_octets: number[];
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
    network: string | null;
}, {
    ip: string;
    ip_octets: number[];
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
    network: string | null;
}>;
export declare const V4IfAddr: z.ZodObject<{
    ip: z.ZodString;
    ip_octets: z.ZodArray<z.ZodNumber, "many">;
    broadcast: z.ZodNullable<z.ZodString>;
    broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    netmask: z.ZodNullable<z.ZodString>;
    netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    prefix: z.ZodNullable<z.ZodNumber>;
    network: z.ZodNullable<z.ZodString>;
}, "strip", z.ZodTypeAny, {
    ip: string;
    ip_octets: number[];
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
    network: string | null;
}, {
    ip: string;
    ip_octets: number[];
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
    network: string | null;
}>;
export declare type V4IfAddr = z.infer<typeof V4IfAddr>;
export declare type V6IfAddr = z.infer<typeof V6IfAddr>;
export declare const Addr: z.ZodRecord<z.ZodString, z.ZodUnion<[z.ZodObject<{
    ip: z.ZodString;
    ip_octets: z.ZodArray<z.ZodNumber, "many">;
    broadcast: z.ZodNullable<z.ZodString>;
    broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    netmask: z.ZodNullable<z.ZodString>;
    netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    prefix: z.ZodNullable<z.ZodNumber>;
    network: z.ZodNullable<z.ZodString>;
}, "strip", z.ZodTypeAny, {
    ip: string;
    ip_octets: number[];
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
    network: string | null;
}, {
    ip: string;
    ip_octets: number[];
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
    network: string | null;
}>, z.ZodObject<{
    ip: z.ZodString;
    ip_octets: z.ZodArray<z.ZodNumber, "many">;
    broadcast: z.ZodNullable<z.ZodString>;
    broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    netmask: z.ZodNullable<z.ZodString>;
    netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    prefix: z.ZodNullable<z.ZodNumber>;
    network: z.ZodNullable<z.ZodString>;
}, "strip", z.ZodTypeAny, {
    ip: string;
    ip_octets: number[];
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
    network: string | null;
}, {
    ip: string;
    ip_octets: number[];
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
    network: string | null;
}>]>>;
export declare type Addr = z.infer<typeof Addr>;
export declare const NetworkInterface: z.ZodObject<{
    name: z.ZodString;
    v4_addrs: z.ZodArray<z.ZodObject<{
        ip: z.ZodString;
        ip_octets: z.ZodArray<z.ZodNumber, "many">;
        broadcast: z.ZodNullable<z.ZodString>;
        broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
        netmask: z.ZodNullable<z.ZodString>;
        netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
        prefix: z.ZodNullable<z.ZodNumber>;
        network: z.ZodNullable<z.ZodString>;
    }, "strip", z.ZodTypeAny, {
        ip: string;
        ip_octets: number[];
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
        network: string | null;
    }, {
        ip: string;
        ip_octets: number[];
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
        network: string | null;
    }>, "many">;
    v6_addrs: z.ZodArray<z.ZodObject<{
        ip: z.ZodString;
        ip_octets: z.ZodArray<z.ZodNumber, "many">;
        broadcast: z.ZodNullable<z.ZodString>;
        broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
        netmask: z.ZodNullable<z.ZodString>;
        netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
        prefix: z.ZodNullable<z.ZodNumber>;
        network: z.ZodNullable<z.ZodString>;
    }, "strip", z.ZodTypeAny, {
        ip: string;
        ip_octets: number[];
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
        network: string | null;
    }, {
        ip: string;
        ip_octets: number[];
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
        network: string | null;
    }>, "many">;
    mac_addr: z.ZodNullable<z.ZodString>;
    index: z.ZodNumber;
}, "strip", z.ZodTypeAny, {
    name: string;
    v4_addrs: {
        ip: string;
        ip_octets: number[];
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
        network: string | null;
    }[];
    v6_addrs: {
        ip: string;
        ip_octets: number[];
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
        network: string | null;
    }[];
    mac_addr: string | null;
    index: number;
}, {
    name: string;
    v4_addrs: {
        ip: string;
        ip_octets: number[];
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
        network: string | null;
    }[];
    v6_addrs: {
        ip: string;
        ip_octets: number[];
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
        network: string | null;
    }[];
    mac_addr: string | null;
    index: number;
}>;
export declare type NetworkInterface = z.infer<typeof NetworkInterface>;
export declare const IpPortPair: z.ZodObject<{
    ip: z.ZodString;
    port: z.ZodNumber;
}, "strip", z.ZodTypeAny, {
    ip: string;
    port: number;
}, {
    ip: string;
    port: number;
}>;
export declare type IpPortPair = z.infer<typeof IpPortPair>;
//# sourceMappingURL=types.d.ts.map