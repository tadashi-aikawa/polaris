<main style="padding: 30px;">
  {#await userPromise}
    <InlineLoading description="Initializing..." />
  {:then user}
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
          <span style="margin-left: 3px">By name</span>
        </div>
      </Tab>

      <svelte:fragment slot="content">
        <div style="height: calc(100vh - 100px);">
          <TabContent>
            <FreeSearch />
          </TabContent>
          <TabContent>
            <div>hogehoge</div>
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

  import { Response, User } from "~/model/get-current-user";
  import FreeSearch from "~/components/organism/FreeSearch.svelte";
  import { Search20, CriticalGlyph } from "carbon-icons-svelte";

  let userPromise: Promise<User | null> = Promise.resolve(null);

  onMount(() => {
    userPromise = invoke<Response>("get_current_user").then((r) => r.user);
  });
</script>
