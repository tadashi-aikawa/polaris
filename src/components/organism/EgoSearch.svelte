<div style="display: flex; gap: 3px">
  {#each queries as query}
    <Tag type="cyan">{query}</Tag>
  {/each}
  <Button
    size="small"
    icon={Search32}
    on:click={search}
    style="margin-left: auto"
    >Search
  </Button>
</div>
<div style="padding-top: 10px; height: calc(100vh - 100px - 50px);">
  {#await resultPromises}
    <Tabs autoWidth>
      {#each queries as q}
        <Tab>
          <div style="display: flex; align-items: center">
            <span style="margin-right: 3px">{q}</span>
            <Tag size="sm" skelton />
          </div>
        </Tab>
      {/each}
    </Tabs>
    <InlineLoading description="Loading..." />
  {:then result}
    <Tabs autoWidth>
      {#each result as r}
        <Tab>
          <div style="display: flex; align-items: center">
            <span style="margin-right: 3px">{r.query}</span>
            <Tag type="warm-gray" size="sm"
              >{unreadMessages(r.messages).length}</Tag>
          </div>
        </Tab>
      {/each}
      <svelte:fragment slot="content">
        {#each result as r}
          <TabContent>
            <div
              style=" height: calc(100vh - 100px - 50px - 100px); overflow-y: scroll">
              {#each unreadMessages(r.messages) as message}
                <div style="padding: 5px;">
                  <MessageCard {message} on:click:read={handleRead} />
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
  import { DateTime } from "owlelia";
  import { sendNotification } from "@tauri-apps/api/notification";
  import { onDestroy, onMount } from "svelte";

  export let queries: string[];
  export let intervalSec: number;

  let resultPromises: Promise<{ query: string; messages: Message[] }[]> =
    Promise.resolve([]);
  // XXX: やっぱ微妙だな。。
  let readById: { [messageId: string]: DateTime } = {};
  let lastMessageIdByQuery: { [query: string]: Message["id"] } = {};

  $: unreadMessages = (messages: Message[]) =>
    messages.filter((x) => !readById[x.id]);

  const search = () => {
    resultPromises = Promise.all(
      queries.map((query) =>
        invoke<Response>("search_messages", {
          query: `${query} after:${DateTime.today().minusDays(2).displayDate}`,
        }).then((r) => {
          const latestMessageId = r.messages?.[0]?.id;
          if (lastMessageIdByQuery[query] !== latestMessageId) {
            sendNotification(`"${query}" の結果に変更がありました`);
            lastMessageIdByQuery[query] = latestMessageId;
          }
          return {
            query,
            messages: r.messages,
          };
        })
      )
    );
  };

  const handleRead = (event: CustomEvent<Message>) => {
    readById[event.detail.id] = DateTime.now();
  };

  let handler: number;
  onMount(() => {
    handler = window.setInterval(search, intervalSec * 1000);
    search();
  });

  onDestroy(() => {
    window.clearInterval(handler);
  });
</script>
