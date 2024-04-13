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
  let interfaces: Array<NetworkInterface> = [];
  $: addresses = interfaces
    .map((iface) => {
      const v4addrs = iface.v4_addrs.map((v4) => ({
        index: iface.index,
        name: iface.name,
        mac_addr: iface.mac_addr,
        ip: v4.ip,
        netmask: v4.netmask,
        type: 'V4'
      }));
      const v6addrs = iface.v6_addrs.map((v6) => ({
        index: iface.index,
        name: iface.name,
        mac_addr: iface.mac_addr,
        ip: v6.ip,
        netmask: v6.netmask,
        type: 'V6'
      }));
      return [...v4addrs, ...v6addrs];
    })
    .flat();

  function getInterfacesOnClick() {
    getInterfaces().then((ifaces: Array<Object>) => {
      const parsed = z.array(NetworkInterface).safeParse(ifaces);
      if (parsed.success) {
        data = JSON.stringify(parsed.data, null, 2);
        interfaces = parsed.data;
        // sort interfaces by index
        interfaces.sort((a, b) => a.index - b.index);
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
        interfaces = parsed.data;
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

<div class="flex flex-col space-y-5 h-full py-5">
  {#if error}
    <div class="alert alert-error">
      <span>{error}</span>
    </div>
  {/if}

  <br />
  <div class="grid grid-cols-4 gap-4">
    <button class="flex-none btn" on:click={getInterfacesOnClick}
      >Get All Interfaces</button
    >
    <button class="flex-none btn" on:click={getNonEmptyInterfacesOnClick}
      >Get Non Empty Interfaces</button
    >
  </div>
  {#if data}
    <div class="card h-96 bg-neutral w-full overflow-auto">
      <div class="card-body">
        <pre>{data}</pre>
      </div>
    </div>
  {/if}
  {#if interfaces && interfaces.length > 0}
    <div class="grow overflow-x-auto">
      <table class="table">
        <!-- head -->
        <thead>
          <tr>
            <th>Index</th>
            <th>Type</th>
            <th>Interface Name</th>
            <th>Mac Address</th>
            <th>IP</th>
            <th>Netmask</th>
          </tr>
        </thead>
        <tbody>
          {#each addresses as iface}
            <tr>
              <td>{iface.index}</td>
              <td>{iface.type}</td>
              <td>{iface.name}</td>
              <td>{iface.mac_addr}</td>
              <td>{iface.ip}</td>
              <td>{iface.netmask}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
