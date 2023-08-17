import { z } from "zod";
export declare const V6IfAddr: z.ZodObject<{
    ip: z.ZodString;
    broadcast: z.ZodNullable<z.ZodString>;
    broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    netmask: z.ZodNullable<z.ZodString>;
    netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
}, "strip", z.ZodTypeAny, {
    ip: string;
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
}, {
    ip: string;
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
}>;
export declare const V4IfAddr: z.ZodObject<{
    ip: z.ZodString;
    broadcast: z.ZodNullable<z.ZodString>;
    broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    netmask: z.ZodNullable<z.ZodString>;
    netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    prefix: z.ZodNullable<z.ZodNumber>;
}, "strip", z.ZodTypeAny, {
    ip: string;
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
}, {
    ip: string;
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
}>;
export declare type V4IfAddr = z.infer<typeof V4IfAddr>;
export declare type V6IfAddr = z.infer<typeof V6IfAddr>;
export declare const Addr: z.ZodRecord<z.ZodString, z.ZodUnion<[z.ZodObject<{
    ip: z.ZodString;
    broadcast: z.ZodNullable<z.ZodString>;
    broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    netmask: z.ZodNullable<z.ZodString>;
    netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    prefix: z.ZodNullable<z.ZodNumber>;
}, "strip", z.ZodTypeAny, {
    ip: string;
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
}, {
    ip: string;
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
    prefix: number | null;
}>, z.ZodObject<{
    ip: z.ZodString;
    broadcast: z.ZodNullable<z.ZodString>;
    broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    netmask: z.ZodNullable<z.ZodString>;
    netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
}, "strip", z.ZodTypeAny, {
    ip: string;
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
}, {
    ip: string;
    broadcast: string | null;
    broadcast_octets: number[] | null;
    netmask: string | null;
    netmask_octets: number[] | null;
}>]>>;
export declare type Addr = z.infer<typeof Addr>;
export declare const NetworkInterface: z.ZodObject<{
    name: z.ZodString;
    v4_addrs: z.ZodArray<z.ZodObject<{
        ip: z.ZodString;
        broadcast: z.ZodNullable<z.ZodString>;
        broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
        netmask: z.ZodNullable<z.ZodString>;
        netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
        prefix: z.ZodNullable<z.ZodNumber>;
    }, "strip", z.ZodTypeAny, {
        ip: string;
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
    }, {
        ip: string;
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
    }>, "many">;
    v6_addrs: z.ZodArray<z.ZodObject<{
        ip: z.ZodString;
        broadcast: z.ZodNullable<z.ZodString>;
        broadcast_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
        netmask: z.ZodNullable<z.ZodString>;
        netmask_octets: z.ZodNullable<z.ZodArray<z.ZodNumber, "many">>;
    }, "strip", z.ZodTypeAny, {
        ip: string;
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
    }, {
        ip: string;
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
    }>, "many">;
    mac_addr: z.ZodNullable<z.ZodString>;
    index: z.ZodNumber;
}, "strip", z.ZodTypeAny, {
    name: string;
    v4_addrs: {
        ip: string;
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
    }[];
    v6_addrs: {
        ip: string;
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
    }[];
    mac_addr: string | null;
    index: number;
}, {
    name: string;
    v4_addrs: {
        ip: string;
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
        prefix: number | null;
    }[];
    v6_addrs: {
        ip: string;
        broadcast: string | null;
        broadcast_octets: number[] | null;
        netmask: string | null;
        netmask_octets: number[] | null;
    }[];
    mac_addr: string | null;
    index: number;
}>;
export declare type NetworkInterface = z.infer<typeof NetworkInterface>;
