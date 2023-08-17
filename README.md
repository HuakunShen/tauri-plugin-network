# Tauri Plugin network

> This is a Tauri plugin for reading network interface information and scanning network.

## Features

- [x] Retrieve network interface information
- [x] TCP host up detection
- [x] Scan local network ips on specified port using HTTP
  - [x] With optional response keyword detection
  - [x] Batch scanning with multi-threading
- [ ] ICMP scan
- [ ] Network data transmission monitoring
- [ ] Packet sniffing (This is harder on Windows as [pnet](https://crates.io/crates/pnet) on Windows requires installation of WinPcap or npcap)

## Installation

> If you are installing from npm and crate.io package registry, make sure the versions for both packages are the same, otherwise, the API may not match.

### Rust Install

`cargo add tauri-plugin-network` to add the package.

Or add the following to your `Cargo.toml` for the latest unpublished version (not recommanded).

```toml
tauri-plugin-network = { git = "https://github.com/HuakunShen/tauri-plugin-network", branch = "main" }
```

### NPM Install

Run the following to install JavaScript/TypeScript API package.

```bash
npm i tauri-plugin-network-api
# npm add https://github.com/HuakunShen/tauri-plugin-network # or this for latest unpublished version (not recommended)
```

In `main.rs`, add the following to your `tauri::Builder`:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_network::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Third Party Libraries Used

- [`network-interface`](https://crates.io/crates/network-interface)

## API

### TypeScript

All TypeScript APIs can be found in [api.ts](./webview-src/api.ts).

Return type of each API is added. The object structures can be found in [type.ts](./webview-src/type.ts).

Zod was used to define type schema and infer TypeScript types. You can import the types exported from the npm package.

The exported zod schemas can be used to parse data and make sure the data returned from rust APIs match the desired structure defined in schema.

#### Get Interface Info

```typescript
import { getInterfaces, NetworkInterface } from "tauri-plugin-network-api";

function getInterfacesOnClick() {
  getInterfaces().then((ifaces: Array<Object>) => {
    const parsed = z.array(NetworkInterface).safeParse(ifaces);
    if (parsed.success) {
      data = JSON.stringify(parsed.data, null, 2);
    } else {
      error = parsed.error.toString();
    }
  });
}
```

#### Scanning

```typescript
import {
  isHttpPortOpen,
  isPortTaken,
  findAvailablePort,
  scanOnlineIpPortPairs,
  scanOnlineIpsByPort,
} from "tauri-plugin-network-api";

console.log(await is_http_port_open("127.0.0.1", 8000));
console.log(await isPortTaken(8000));
console.log(await findAvailablePort());
console.log(
  await scanOnlineIpPortPairs([
    { ip: "127.0.0.1", port: 8000 },
    { ip: "192.168.3.6", port: 8000 },
    { ip: "192.168.3.5", port: 8000 },
  ])
);
console.log(
  await scanOnlineIpsByPort(["127.0.0.1", "192.168.3.6", "192.168.1.2"], 8000)
);
```

## Usage

See [SvelteKit Example](./examples/sveltekit/README.md) for an example written with SvelteKit.
