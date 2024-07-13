import { number, object, string, type InferOutput } from "valibot"

export const Ipv4Network = object({
  addr: string(),
  prefix: number()
})
export type Ipv4Network = InferOutput<typeof Ipv4Network>
