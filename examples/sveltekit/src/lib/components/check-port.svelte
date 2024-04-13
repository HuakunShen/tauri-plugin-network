<script lang="ts">
  import { onMount } from "svelte";
  import { isPortTaken } from "tauri-plugin-network-api";

  let port = 1566;
  let taken: boolean | undefined = undefined;
  onMount(async () => {
    taken = await isPortTaken(1566);
  });

  async function check() {
    taken = await isPortTaken(port);
  }
</script>

<h2 class="text-2xl">Check If Port is Taken</h2>
<form on:submit={check}>
  <div class="form-control">
    <label class="label">
      <span class="label-text">Check If Port is Taken</span>
    </label>
    <label class="input-group">
      <span>Check If Port is Taken</span>
      <input
        type="number"
        placeholder="Port Number"
        class="input input-bordered"
        bind:value={port}
        on:input={() => (taken = undefined)}
      />
      <button class="btn btn-primary" type="submit">Check</button>
    </label>
  </div>
  {#if taken !== undefined}
    <p><strong>Port {port}</strong> is {taken ? " " : "not "}taken</p>
  {/if}
</form>
