<main style="padding: 45px;">
  {#await initializePromise}
    <InlineLoading description="Initializing..." />
  {:then result}
    <EgoSearch queries={result.queries} intervalSec={result.interval_sec} includeMe={result.include_me} />
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
    include_me: false,
  });

  onMount(async () => {
    initializePromise = new Promise<Config>(async (resolve, reject) => {
      try {
        await invoke<void>("initialize");
        resolve(await invoke<Config>("fetch_config"));
      } catch (e) {
        reject(e);
      }
    });
  });
</script>
