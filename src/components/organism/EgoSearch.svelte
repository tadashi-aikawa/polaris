<div style="display: flex; gap: 3px">
  {#each queries as query}
    <Tag type="cyan">{query}</Tag>
  {/each}
  <Button
    size="small"
    icon={Search32}
    on:click={handleClickSearch}
    style="margin-left: auto"
    >Search
  </Button>
</div>
<div style="padding-top: 10px; height: calc(100vh - 100px - 50px);">
  {#await resultPromises}
    <InlineLoading description="Loading..." />
  {:then result}
    <Tabs autoWidth>
      {#each result as r}
        <Tab label={r.query} />
      {/each}
      <svelte:fragment slot="content">
        {#each result as r}
          <TabContent>
            <div
              style=" height: calc(100vh - 100px - 50px - 100px); overflow-y: scroll">
              {#each r.messages as message}
                <div style="padding: 5px;">
                  <MessageCard {message} />
                </div>
              {/each}
            </div>
          </TabContent>
        {/each}
      </svelte:fragment>
    </Tabs>
  {:catch error}
    <InlineNotification title="Error" subtitle={error} />
  {/await}
</div>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    Button,
    InlineLoading,
    InlineNotification,
    Tab,
    TabContent,
    Tabs,
    Tag,
  } from "carbon-components-svelte";

  import { Message, Response } from "~/model/search-messages";
  import MessageCard from "~/components/molecules/MessageCard.svelte";
  import { Search32 } from "carbon-icons-svelte";

  export let queries: string[];

  let resultPromises: Promise<{ query: string; messages: Message[] }[]> =
    Promise.resolve([]);

  const handleClickSearch = () => {
    resultPromises = Promise.all(
      queries.map((query) =>
        invoke<Response>("search_messages", { query }).then((r) => ({
          query,
          messages: r.messages,
        }))
      )
    );
  };
</script>
