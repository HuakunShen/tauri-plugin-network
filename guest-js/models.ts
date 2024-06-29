import { z } from "zod";

export const Ipv4Network = z.object({
  addr: z.string(),
  prefix: z.number(),
});
export type Ipv4Network = z.infer<typeof Ipv4Network>;
