import { invoke } from "@tauri-apps/api/tauri";

export * from "./types";

export async function execute() {
  await invoke("plugin:network|execute");
}

export async function getInterfaces(): Promise<Array<any>> {
  return await invoke("plugin:network|get_interfaces");
}
