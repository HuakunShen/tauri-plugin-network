import { z } from "zod";

// export const addr =
export const V6IfAddr = z.object({
  ip: z.string(),
  broadcast: z.string().nullable(),
  broadcast_octets: z.array(z.number()).nullable(),
  netmask: z.string().nullable(),
  netmask_octets: z.number().array().nullable(),
});
export const V4IfAddr = V6IfAddr.merge(
  z.object({ prefix: z.number().nullable() })
);
export type V4IfAddr = z.infer<typeof V4IfAddr>;
export type V6IfAddr = z.infer<typeof V6IfAddr>;
export const Addr = z.record(z.string(), z.union([V4IfAddr, V6IfAddr]));
export type Addr = z.infer<typeof Addr>;
export const NetworkInterface = z.object({
  name: z.string(),
  v4_addrs: V4IfAddr.array(),
  v6_addrs: V6IfAddr.array(),
  mac_addr: z.string().nullable(),
  index: z.number(),
});
export type NetworkInterface = z.infer<typeof NetworkInterface>;
