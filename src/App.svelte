<main style="padding: 30px;">
  {#await initializePromise}
    <InlineLoading description="Initializing..." />
  {:then result}
    <Tabs>
      <Tab>
        <div style="display: flex; align-items: center">
          <Search20 />
          <span style="margin-left: 3px">Free search</span>
        </div>
      </Tab>
      <Tab>
        <div style="display: flex; align-items: center">
          <CriticalGlyph />
          <span style="margin-left: 3px">Ego search</span>
        </div>
      </Tab>

      <svelte:fragment slot="content">
        <div style="height: calc(100vh - 100px);">
          <TabContent>
            <FreeSearch />
          </TabContent>
          <TabContent>
            <EgoSearch queries={result.queries} />
          </TabContent>
        </div>
      </svelte:fragment>
    </Tabs>
  {:catch error}
    <InlineNotification title="Error" subtitle={error} />
  {/await}
</main>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  import {
    Tabs,
    Tab,
    TabContent,
    InlineLoading,
    InlineNotification,
  } from "carbon-components-svelte";

  import FreeSearch from "~/components/organism/FreeSearch.svelte";
  import { Search20, CriticalGlyph } from "carbon-icons-svelte";
  import EgoSearch from "~/components/organism/EgoSearch.svelte";

  let initializePromise: Promise<{ queries: string[] }> = Promise.resolve({
    queries: [],
  });

  onMount(async () => {
    initializePromise = new Promise(async (resolve, reject) => {
      try {
        await invoke<void>("initialize");
        const queries = await invoke<{ queries: string[] }>("fetch_queries");
        resolve({ queries });
      } catch (e) {
        reject(e);
      }
    });
  });
</script>
