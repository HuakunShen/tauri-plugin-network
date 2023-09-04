<script lang="ts">
  import { z } from "zod";
  import {
    getInterfaces,
    getNonEmptyInterfaces,
    NetworkInterface,
    isHttpPortOpen,
    isPortTaken,
    findAvailablePort,
    scanOnlineIpPortPairs,
    scanOnlineIpsByPort,
    nonLocalhostNetworks,
    localServerIsRunning,
    scanLocalNetworkOnlineHostsByPort,
  } from "tauri-plugin-network-api";
  import { onMount } from "svelte";

  let data: string = "";
  let error: string = "";

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
  function getNonEmptyInterfacesOnClick() {
    getNonEmptyInterfaces().then((ifaces: Array<Object>) => {
      const parsed = z.array(NetworkInterface).safeParse(ifaces);
      if (parsed.success) {
        data = JSON.stringify(parsed.data, null, 2);
      } else {
        error = parsed.error.toString();
      }
    });
  }

  onMount(async () => {
    // const open = await is_http_port_open("127.0.0.1", 8000);
    // console.log(open);
    // console.log(await isPortTaken(8000));
    // console.log(await findAvailablePort());
    console.log(
      await scanOnlineIpPortPairs(
        [
          { ip: "127.0.0.1", port: 8000 },
          { ip: "192.168.3.6", port: 8000 },
          { ip: "192.168.3.5", port: 8000 },
        ],
        "CrossCopy"
      )
    );
    console.log(
      await scanOnlineIpsByPort(
        ["127.0.0.1", "192.168.3.6", "192.168.1.2"],
        8000,
        "CrossCopy"
      )
    );
    console.log("Non Localhost Networks", await nonLocalhostNetworks());
    console.log("Local Server is Running", await localServerIsRunning(8000));
    console.log(
      "Scan Local Network for service",
      await scanLocalNetworkOnlineHostsByPort(8000, "CrossCopy")
    );
  });
</script>

<div class="flex flex-col h-full">
  {#if error}
    <div class="alert alert-error">
      <span>{error}</span>
    </div>
  {/if}
  <div class="grid grid-cols-4 gap-4">
    <button class="flex-none btn" on:click={getInterfacesOnClick}
      >Get All Interfaces</button
    >
    <button class="flex-none btn" on:click={getNonEmptyInterfacesOnClick}
      >Get Non Empty Interfaces</button
    >
  </div>
  {#if data}
    <div class="grow card bg-neutral w-full overflow-auto mt-3">
      <div class="card-body">
        <pre>{data}</pre>
      </div>
    </div>
  {/if}
</div>
