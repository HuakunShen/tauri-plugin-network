import {
  array,
  literal,
  nullable,
  number,
  object,
  optional,
  record,
  string,
  union,
  type InferOutput
} from "valibot"

export const V6IfAddr = object({
  ip: string(),
  ip_octets: array(number()),
  broadcast: nullable(string()),
  broadcast_octets: nullable(array(number())),
  netmask: nullable(string()),
  netmask_octets: nullable(array(number())),
  prefix: nullable(number()),
  network: nullable(string())
})
export const V4IfAddr = V6IfAddr
export type V4IfAddr = InferOutput<typeof V4IfAddr>
export type V6IfAddr = InferOutput<typeof V6IfAddr>
export const Addr = record(string(), union([V4IfAddr, V6IfAddr]))
export type Addr = InferOutput<typeof Addr>
export const NetworkInterface = object({
  name: string(),
  v4_addrs: array(V4IfAddr),
  v6_addrs: array(V6IfAddr),
  mac_addr: nullable(string()),
  index: number()
})
export type NetworkInterface = InferOutput<typeof NetworkInterface>

export const IpPortPair = object({
  ip: string(),
  port: number()
})
export type IpPortPair = InferOutput<typeof IpPortPair>

export const HttpScanOptions = object({
  port: number(),
  keyword: optional(string()),
  route: optional(string()),
  protocol: optional(union([literal("http"), literal("https")])),
  statusCode: optional(number())
})
export type HttpScanOptions = InferOutput<typeof HttpScanOptions>
