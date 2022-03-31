<!--suppress ALL -->
<div style="display: flex; gap: 5px">
  <Button size="small" icon={Search32} on:click={searchAll}>Search</Button>
  <Button
    kind="danger"
    size="small"
    icon={CheckmarkOutline32}
    on:click={markAsReadAll}>Mark as read all</Button>
</div>
<div
  style="padding-top: 20px; width: 960px; height: calc(100vh - 100px - 50px);">
  <Tabs autoWidth>
    {#each results as r}
      <Tab disabled={r.value.messages.length === 0}>
        <div style="display: flex; align-items: center">
          <span style="margin-right: 3px">{r.value.query}</span>
          {#if r.loading}
            <InlineLoading />
          {:else}
            <Tag type="cyan" size="sm" disabled={r.value.messages.length === 0}
              >{unreadMessages(r.value.messages).length}</Tag>
          {/if}
        </div>
      </Tab>
    {/each}

    <svelte:fragment slot="content">
      {#each results as r}
        {#if r.error}
          <InlineNotification title="Error" subtitle={r.error.message} />
        {/if}
        <TabContent skelton={r.loading}>
          <div
            style="width: 960px; height: calc(100vh - 100px - 50px - 100px); overflow-y: scroll">
            {#each unreadMessages(r.value.messages) as message, i (message)}
              <div
                style="padding: 5px;"
                animate:flip={{ duration: 500 }}
                in:fade
                out:fly={{ x: 100 }}>
                <MessageCard {message} on:click:read={handleRead} />
              </div>
            {/each}
          </div>
        </TabContent>
      {/each}
    </svelte:fragment>
  </Tabs>
</div>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    Button,
    InlineNotification,
    Tab,
    TabContent,
    Tabs,
    Tag,
    InlineLoading,
  } from "carbon-components-svelte";

  import { Message, Response } from "~/model/search-messages";
  import MessageCard from "~/components/molecules/MessageCard.svelte";
  import { Search32, CheckmarkOutline32 } from "carbon-icons-svelte";
  import { DateTime, fromPromise, LiquidValue } from "owlelia";
  import { sendNotification } from "@tauri-apps/api/notification";
  import { onDestroy, onMount } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { flip } from "svelte/animate";

  export let queries: string[];
  export let intervalSec: number;

  type LValue = { query: string; messages: Message[] };

  // XXX: やっぱ微妙だな。。
  let readById: { [messageId: string]: DateTime } = {};
  let lastMessageIdByQuery: { [query: string]: Message["id"] } = {};
  let results: LiquidValue<LValue>[] = queries.map(
    (query) => new LiquidValue({ query, messages: [] })
  );

  $: unreadMessages = (messages: Message[]) =>
    messages.filter((x) => !readById[x.id]);

  const search = async (
    lv: LiquidValue<LValue>
  ): Promise<LiquidValue<LValue>> => {
    await lv.load(() => {
      const query = lv.value.query;
      return fromPromise(
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
      );
    });
    return lv;
  };

  const handleRead = (event: CustomEvent<Message>) => {
    readById[event.detail.id] = DateTime.now();
  };

  const searchAll = async () => {
    for (let i = 0; i < results.length; i++) {
      results[i] = await search(results[i]);
    }
  };

  const markAsReadAll = async () => {
    unreadMessages(results.flatMap((x) => x.value.messages)).forEach((m) => {
      readById[m.id] = DateTime.now();
    });
  };

  let handler: number;
  onMount(() => {
    handler = window.setInterval(searchAll, intervalSec * 1000);
    searchAll();
  });

  onDestroy(() => {
    window.clearInterval(handler);
  });
</script>
