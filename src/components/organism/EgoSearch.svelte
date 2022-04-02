<!--suppress ALL -->
<div style="display: flex; gap: 5px">
  <Button size="small" icon={Search32} on:click={handleClickSearch}
    >Search</Button>
  <Button
    kind="danger"
    size="small"
    icon={CheckmarkOutline32}
    on:click={markAsReadAll}>Mark as read all</Button>
</div>
<div
  style="padding-top: 20px;  height: calc(100vh - 100px - 50px);">
  <Tabs autoWidth>
    {#each results as r}
      <Tab disabled={unreadMessages(r.item.messages).length === 0}>
        <div style="display: flex; align-items: center; height: 26px">
          <span style="margin-right: 3px;">{r.item.query}</span>
          {#if r.loading}
            <InlineLoading />
          {:else if unreadMessages(r.item.messages).length > 0}
            <Tag type="cyan" size="sm"
              >{unreadMessages(r.item.messages).length}</Tag>
          {/if}
        </div>
      </Tab>
    {/each}

    <svelte:fragment slot="content">
      {#each results as r}
        {#if r.error}
          <InlineNotification title="Error" subtitle={r.error} />
        {/if}
        <TabContent>
          <div
            style=" height: calc(100vh - 100px - 50px - 100px); overflow-y: scroll">
            {#each unreadMessages(r.item.messages) as message, i (message)}
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
  import { sleep } from "~/utils/time";
  import { Search32, CheckmarkOutline32 } from "carbon-icons-svelte";
  import { AsyncResult, DateTime, fromPromise, Nullable } from "owlelia";
  import { sendNotification } from "@tauri-apps/api/notification";
  import { onMount } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { flip } from "svelte/animate";

  export let queries: string[];
  export let intervalSec: number;
  export let includeMe: boolean;

  type Item = { query: string; messages: Message[] };
  type LiquidValue = {
    item: Item;
    loading: boolean;
    error: Nullable<string>;
  };

  let readById: { [messageId: string]: DateTime } = {};
  let lastMessageIdByQuery: { [query: string]: Message["id"] } = {};
  let results: LiquidValue[] = queries.map((query) => ({
    item: { query, messages: [] },
    loading: false,
    error: null,
  }));

  $: unreadMessages = (messages: Message[]) =>
    messages.filter((x) => !readById[x.id]);

  const handleRead = (event: CustomEvent<Message>) => {
    readById[event.detail.id] = DateTime.now();
  };

  const markAsReadAll = async () => {
    unreadMessages(results.flatMap((x) => x.item.messages)).forEach((m) => {
      readById[m.id] = DateTime.now();
    });
  };

  const searchItem = async (query: string): Promise<Item> => {
    return invoke<Response>("search_messages", {
      query: `${query} after:${DateTime.today().minusDays(2).displayDate}`,
      excludeMe: !includeMe,
    }).then((r) => ({
        query,
        messages: r.messages,
      }));
  };

  const search = async (i: number, shouldNotify: boolean) => {
    results[i].loading = true;
    try {
      const query = results[i].item.query
      const item = await searchItem(query);
      results[i].item = item

      const latestMessageId = item.messages?.[0]?.id;
      if (shouldNotify && lastMessageIdByQuery[query] !== latestMessageId && !readById[latestMessageId]) {
        sendNotification(`"${query}" に関する新しいメッセージを見つけました🦋`);
      }
      lastMessageIdByQuery[query] = latestMessageId;
    } catch (e) {
      results[i].error = e;
    }
    results[i].loading = false;
  };

  const searchAll = async () => {
    for (let i = 0; i < results.length; i++) {
      await search(i, false);
    }
  };

  const handleClickSearch = searchAll;

  onMount(async () => {
    const eachIntervalSec = intervalSec / results.length;
    const endlessIntervalSearch = async (i: number) => {
      await search(i, true);
      await sleep(intervalSec * 1000)
      endlessIntervalSearch(i);
    }

    for (let i = 0; i < results.length; i++) {
      await sleep(eachIntervalSec * 1000);
      endlessIntervalSearch(i)
    }
  });
</script>
