<!--suppress ALL -->
<div style="display: flex; gap: 5px">
  <Button size="small" icon={Search32} on:click={handleClickSearch}
    >Search</Button>
  <Button
    kind="danger"
    size="small"
    icon={CheckmarkOutline32}
    on:click={handleClickMarkAsReadAll}>Mark all as read</Button>
  <Button size="small" icon={Search32} on:click={handleClickReloadConfig}
    >Reload config</Button>
</div>
<div style="padding-top: 20px;  height: calc(100vh - 100px - 50px);">
  <Tabs autoWidth>
    {#each results as r}
      <Tab>
        <div
          style="display: flex; align-items: center; height: 26px; padding-bottom: 10px;">
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
      {#each results as r, i}
        {#if r.error}
          <InlineNotification title="Error" subtitle={r.error} />
        {/if}
        <TabContent>
          <div style="margin-bottom: 10px">
            <Button
              kind="ghost"
              size="small"
              icon={Search32}
              on:click={() => handleClickSearchByCurrentQuery(i)}
              >Search by a current query</Button>
            {#if unreadMessages(r.item.messages).length > 0}
              <Button
                kind="danger-ghost"
                size="small"
                icon={CheckmarkOutline32}
                on:click={() => handleClickMarkAsReadItem(r.item)}
                >Mark messages in this tab as read</Button>
            {/if}
          </div>
          <div
            style=" height: calc(100vh - 100px - 50px - 100px); overflow-y: scroll">
            {#if unreadMessages(r.item.messages).length === 0}
              <div
                transition:fade
                style="display: flex; justify-content: center; align-items: center; flex-direction: column; padding-top: 60px">
                <img
                  src="https://github.com/tadashi-aikawa/vigilancia/raw/master/src-tauri/icons/128x128@2x.png" />
                {#if r.item.lastSearchDate}
                  <span style="font-size: 18px; color: mediumpurple">
                    There are no new beneficial messages since
                    {r.item.lastSearchDate.displayDateTime}
                  </span>
                {:else}
                  <span style="font-size: 18px; color: mediumpurple">
                    You should wait until starting the initial search or click
                    the search button immediately
                  </span>
                {/if}
              </div>
            {/if}
            {#each unreadMessages(r.item.messages) as message, i (message)}
              <div
                style="padding: 5px;"
                animate:flip={{ duration: 500 }}
                in:fade
                out:fly={{ x: 100 }}>
                <MessageCard {message} on:click:read={handleClickMarkAsRead} />
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
  import { onDestroy, onMount } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { flip } from "svelte/animate";

  type Item = { query: string; messages: Message[]; lastSearchDate?: DateTime };
  type LiquidValue = {
    item: Item;
    loading: boolean;
    error: Nullable<string>;
  };

  let config: Config = {
    queries: [],
    interval_sec: 60 * 10,
    include_me: false,
  };

  let readById: { [messageId: string]: DateTime } = {};
  let lastMessageIdByQuery: { [query: string]: Message["id"] } = {};
  let results: LiquidValue[] = [];

  $: unreadMessages = (messages: Message[]) =>
    messages.filter((x) => !readById[x.id]);

  const markAsRead = async (message: Message) => {
    readById[message.id] = DateTime.now();
  };
  const markAsReadItem = async (item: Item) => {
    unreadMessages(item.messages).forEach(markAsRead);
  };

  const handleClickMarkAsRead = (event: CustomEvent<Message>) => {
    markAsRead(event.detail);
  };

  const handleClickMarkAsReadItem = (item: Item) => {
    markAsReadItem(item);
  };
  const handleClickMarkAsReadAll = async () => {
    results.map((x) => x.item).forEach(markAsReadItem);
  };

  const searchItem = async (query: string): Promise<Item> => {
    return invoke<Response>("search_messages", {
      query: `${query} after:${DateTime.today().minusDays(2).displayDate}`,
      excludeMe: !config.include_me,
    }).then((r) => {
      return {
        query,
        messages: r.messages,
        lastSearchDate: DateTime.now(),
      };
    });
  };

  const search = async (i: number, shouldNotify: boolean) => {
    results[i].loading = true;
    try {
      const query = results[i].item.query;
      const item = await searchItem(query);
      results[i].item = item;

      const latestMessageId = item.messages?.[0]?.id;
      if (
        shouldNotify &&
        lastMessageIdByQuery[query] !== latestMessageId &&
        !readById[latestMessageId]
      ) {
        sendNotification(`"${query}" に関する新しいメッセージを見つけました🦋`);
      }
      lastMessageIdByQuery[query] = latestMessageId;
    } catch (e) {
      results[i].error = e;
    }
    results[i].loading = false;
  };

  const handleClickSearchByCurrentQuery = async (i: number) => {
    await search(i, false);
  };

  const searchAll = async () => {
    for (let i = 0; i < results.length; i++) {
      await search(i, false);
    }
  };

  const loadConfig = async () => {
    config = await invoke<Config>("get_config");
    results = config.queries.map((query) => ({
      item: { query, messages: [] },
      loading: false,
      error: null,
    }));
  };

  const handleClickReloadConfig = async () => {
    timeoutHandlers.forEach((x) => window.clearTimeout(x));
    await invoke<void>("reload_config");
    await loadConfig();
  };

  const handleClickSearch = searchAll;

  let timeoutHandlers: number[] = [];
  onMount(async () => {
    await loadConfig();

    const eachIntervalSec = config.interval_sec / results.length;
    const endlessIntervalSearch = async (i: number) => {
      await search(i, true);
      timeoutHandlers.push(
        window.setTimeout(() => {
          endlessIntervalSearch(i);
        }, config.interval_sec * 1000)
      );
    };

    for (let i = 0; i < results.length; i++) {
      await sleep(eachIntervalSec * 1000);
      endlessIntervalSearch(i);
    }
  });

  onDestroy(() => {
    timeoutHandlers.forEach((x) => window.clearTimeout(x));
  });
</script>
