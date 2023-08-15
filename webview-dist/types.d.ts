import { z } from "zod";
export declare const V4IfAddr: z.ZodObject<{
    ip: z.ZodString;
    broadcast: z.ZodNullable<z.ZodString>;
    netmask: z.ZodString;
}, "strip", z.ZodTypeAny, {
    ip: string;
    broadcast: string | null;
    netmask: string;
}, {
    ip: string;
    broadcast: string | null;
    netmask: string;
}>;
export declare type V4IfAddr = z.infer<typeof V4IfAddr>;
export declare const V6IfAddr: z.ZodObject<{
    ip: z.ZodString;
    broadcast: z.ZodNullable<z.ZodString>;
    netmask: z.ZodString;
}, "strip", z.ZodTypeAny, {
    ip: string;
    broadcast: string | null;
    netmask: string;
}, {
    ip: string;
    broadcast: string | null;
    netmask: string;
}>;
export declare type V6IfAddr = z.infer<typeof V6IfAddr>;
export declare const Addr: z.ZodRecord<z.ZodString, z.ZodUnion<[z.ZodObject<{
    ip: z.ZodString;
    broadcast: z.ZodNullable<z.ZodString>;
    netmask: z.ZodString;
}, "strip", z.ZodTypeAny, {
    ip: string;
    broadcast: string | null;
    netmask: string;
}, {
    ip: string;
    broadcast: string | null;
    netmask: string;
}>, z.ZodObject<{
    ip: z.ZodString;
    broadcast: z.ZodNullable<z.ZodString>;
    netmask: z.ZodString;
}, "strip", z.ZodTypeAny, {
    ip: string;
    broadcast: string | null;
    netmask: string;
}, {
    ip: string;
    broadcast: string | null;
    netmask: string;
}>]>>;
export declare type Addr = z.infer<typeof Addr>;
export declare const NetworkInterface: z.ZodObject<{
    name: z.ZodString;
    addr: z.ZodArray<z.ZodRecord<z.ZodString, z.ZodUnion<[z.ZodObject<{
        ip: z.ZodString;
        broadcast: z.ZodNullable<z.ZodString>;
        netmask: z.ZodString;
    }, "strip", z.ZodTypeAny, {
        ip: string;
        broadcast: string | null;
        netmask: string;
    }, {
        ip: string;
        broadcast: string | null;
        netmask: string;
    }>, z.ZodObject<{
        ip: z.ZodString;
        broadcast: z.ZodNullable<z.ZodString>;
        netmask: z.ZodString;
    }, "strip", z.ZodTypeAny, {
        ip: string;
        broadcast: string | null;
        netmask: string;
    }, {
        ip: string;
        broadcast: string | null;
        netmask: string;
    }>]>>, "many">;
    mac_addr: z.ZodNullable<z.ZodString>;
    index: z.ZodNumber;
}, "strip", z.ZodTypeAny, {
    name: string;
    addr: Record<string, {
        ip: string;
        broadcast: string | null;
        netmask: string;
    }>[];
    mac_addr: string | null;
    index: number;
}, {
    name: string;
    addr: Record<string, {
        ip: string;
        broadcast: string | null;
        netmask: string;
    }>[];
    mac_addr: string | null;
    index: number;
}>;
export declare type NetworkInterface = z.infer<typeof NetworkInterface>;
