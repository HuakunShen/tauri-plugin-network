<script lang="ts">
  import Scan from "$lib/components/scan.svelte";
  import CheckPort from "$lib/components/check-port.svelte";
</script>

<div class="flex-col space-y-4">
  <CheckPort />
  <Scan />
</div>
