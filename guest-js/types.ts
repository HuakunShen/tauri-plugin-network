import * as v from "valibot";

export const V6IfAddr = v.object({
  ip: v.string(),
  ip_octets: v.array(v.number()),
  broadcast: v.nullable(v.string()),
  broadcast_octets: v.nullable(v.array(v.number())),
  netmask: v.nullable(v.string()),
  netmask_octets: v.nullable(v.array(v.number())),
  prefix: v.nullable(v.number()),
  network: v.nullable(v.string()),
});
export const V4IfAddr = V6IfAddr;
export type V4IfAddr = v.InferOutput<typeof V4IfAddr>;
export type V6IfAddr = v.InferOutput<typeof V6IfAddr>;
export const Addr = v.record(v.string(), v.union([V4IfAddr, V6IfAddr]));
export type Addr = v.InferOutput<typeof Addr>;
export const NetworkInterface = v.object({
  name: v.string(),
  v4_addrs: v.array(V4IfAddr),
  v6_addrs: v.array(V6IfAddr),
  mac_addr: v.nullable(v.string()),
  index: v.number(),
});
export type NetworkInterface = v.InferOutput<typeof NetworkInterface>;

export const IpPortPair = v.object({
  ip: v.string(),
  port: v.number(),
});
export type IpPortPair = v.InferOutput<typeof IpPortPair>;

export const HttpScanOptions = v.object({
  port: v.number(),
  keyword: v.optional(v.string()),
  route: v.optional(v.string()),
  protocol: v.optional(v.union([v.literal("http"), v.literal("https")])),
  statusCode: v.optional(v.number()),
});
export type HttpScanOptions = v.InferOutput<typeof HttpScanOptions>;
