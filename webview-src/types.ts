import { z } from "zod";

// export const addr =
export const V4IfAddr = z.object({
  ip: z.string(),
  broadcast: z.nullable(z.string()),
  netmask: z.string(),
});
export type V4IfAddr = z.infer<typeof V4IfAddr>;
export const V6IfAddr = V4IfAddr;
export type V6IfAddr = z.infer<typeof V6IfAddr>;
export const Addr = z.record(z.string(), z.union([V4IfAddr, V6IfAddr]));
export type Addr = z.infer<typeof Addr>;
export const NetworkInterface = z.object({
  name: z.string(),
  addr: z.array(Addr),
  mac_addr: z.nullable(z.string()),
  index: z.number(),
});
export type NetworkInterface = z.infer<typeof NetworkInterface>;
