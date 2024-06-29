import { z } from "zod"

export const V6IfAddr = z.object({
  ip: z.string(),
  ip_octets: z.number().array(),
  broadcast: z.string().nullable(),
  broadcast_octets: z.array(z.number()).nullable(),
  netmask: z.string().nullable(),
  netmask_octets: z.number().array().nullable(),
  prefix: z.number().nullable(),
  network: z.string().nullable(),
})
export const V4IfAddr = V6IfAddr
export type V4IfAddr = z.infer<typeof V4IfAddr>
export type V6IfAddr = z.infer<typeof V6IfAddr>
export const Addr = z.record(z.string(), z.union([V4IfAddr, V6IfAddr]))
export type Addr = z.infer<typeof Addr>
export const NetworkInterface = z.object({
  name: z.string(),
  v4_addrs: V4IfAddr.array(),
  v6_addrs: V6IfAddr.array(),
  mac_addr: z.string().nullable(),
  index: z.number(),
})
export type NetworkInterface = z.infer<typeof NetworkInterface>

export const IpPortPair = z.object({
  ip: z.string(),
  port: z.number(),
})
export type IpPortPair = z.infer<typeof IpPortPair>

export const HttpScanOptions = z.object({
  port: z.number(),
  keyword: z.string().optional(),
  route: z.string().optional(),
  protocol: z.union([z.literal("http"), z.literal("https")]).optional(),
  statusCode: z.number().optional(),
})
export type HttpScanOptions = z.infer<typeof HttpScanOptions>
