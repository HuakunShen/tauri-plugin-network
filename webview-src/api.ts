import { invoke } from "@tauri-apps/api/tauri";
import { NetworkInterface } from "./types";
export * from "./types";

export async function getInterfaces(): Promise<Array<NetworkInterface>> {
  return await invoke("plugin:network|get_interfaces");
}
