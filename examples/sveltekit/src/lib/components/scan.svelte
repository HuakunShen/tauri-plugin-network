<script lang="ts">
  import { onMount } from "svelte"
  import { scanLocalNetworkOnlineHostsByPort, type IpPortPair } from "tauri-plugin-network-api"

  let port = 1566
  let keyword: string | undefined = undefined
  let route: string | undefined = undefined
  let statusCode: number | undefined = undefined
  let ipportpairs: IpPortPair[] = []
  let loading = false

  onMount(async () => {
    ipportpairs = await scanLocalNetworkOnlineHostsByPort({ port: 1566 })
  })

  async function check() {
    loading = true
    ipportpairs = await scanLocalNetworkOnlineHostsByPort({ port, keyword, route, statusCode })
    loading = false
  }
  $: {
    if (keyword !== undefined && keyword.length === 0) {
      keyword = undefined
    }
  }
</script>

<h2 class="text-2xl">Scan Online HTTP Service</h2>
<form on:submit={check} class="flex flex-col space-y-4">
  <div class="form-control">
    <label class="label">
      <span class="label-text">Service Keyword</span>
    </label>
    <label class="input-group">
      <span>Keyword</span>
      <input type="text" class="input input-bordered" bind:value={keyword} />
    </label>
  </div>
  <div class="form-control">
    <label class="label">
      <span class="label-text">Service Route</span>
    </label>
    <label class="input-group">
      <span>Route</span>
      <input type="text" class="input input-bordered" bind:value={route} />
    </label>
  </div>
  <div class="form-control">
    <label class="label">
      <span class="label-text">Expected Status Code</span>
    </label>
    <label class="input-group">
      <span>Status Code</span>
      <input type="number" class="input input-bordered" bind:value={statusCode} />
    </label>
  </div>

  <div class="form-control">
    <label class="label">
      <span class="label-text">Scan Local Network</span>
    </label>
    <label class="input-group">
      <span>Scan Local Network</span>
      <input type="number" class="input input-bordered" placeholder="Port Number" bind:value={port} />
      <button class="btn btn-primary" type="submit">Check</button>
    </label>
  </div>

  <ol class="list">
    {#if loading}
      <span class="loading loading-spinner loading-md" />
    {:else if ipportpairs.length === 0}
      <p>No service found in local network</p>
    {:else}
      {#each ipportpairs as pair, idx}
        <li>
          <span>{idx}</span>
          <span class="flex-auto">{pair.ip}:{pair.port}</span>
        </li>
      {/each}
    {/if}
  </ol>
</form>
