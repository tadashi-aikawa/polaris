<main style="padding: 45px;">
  {#await initializePromise}
    <InlineLoading description="Initializing..." />
  {:then result}
    <EgoSearch queries={result.queries} intervalSec={result.interval_sec} />
  {:catch error}
    <InlineNotification title="Error" subtitle={error} />
  {/await}
</main>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  import { InlineLoading, InlineNotification } from "carbon-components-svelte";

  import EgoSearch from "~/components/organism/EgoSearch.svelte";
  import type { Config } from "~/model/config";

  let initializePromise: Promise<Config> = Promise.resolve({
    queries: [],
    interval_sec: 60 * 10,
  });

  onMount(async () => {
    initializePromise = new Promise<Config>(async (resolve, reject) => {
      try {
        await invoke<void>("initialize");
        const config = await invoke<Config>("fetch_config");
        resolve({ queries: config.queries, interval_sec: config.interval_sec });
      } catch (e) {
        reject(e);
      }
    });
  });
</script>
