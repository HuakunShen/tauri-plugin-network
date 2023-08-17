<script lang="ts">
  import { z } from "zod";
  import { getInterfaces, NetworkInterface } from "tauri-plugin-network-api";

  let data: string = "";
  let error: string = "";

  function getInterfacesOnClick() {
    getInterfaces().then((ifaces: Array<Object>) => {
      console.log(ifaces);
      const parsed = z.array(NetworkInterface).safeParse(ifaces);
      if (parsed.success) {
        data = JSON.stringify(parsed.data, null, 2);
      } else {
        error = parsed.error.toString();
      }
    });
  }
</script>

<div class="flex flex-col h-full">
  {#if error}
    <div class="alert alert-error">
      <span>{error}</span>
    </div>
  {/if}
  <button class="flex-none btn" on:click={getInterfacesOnClick}
    >Get All Interfaces</button
  >
  {#if data}
    <div class="grow card bg-neutral w-full overflow-auto mt-3">
      <div class="card-body">
        <pre>{data}</pre>
      </div>
    </div>
  {/if}
</div>
