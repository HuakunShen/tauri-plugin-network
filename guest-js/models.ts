import * as v from "valibot";

export const Ipv4Network = v.object({
  addr: v.string(),
  prefix: v.number(),
});
export type Ipv4Network = v.InferOutput<typeof Ipv4Network>;
