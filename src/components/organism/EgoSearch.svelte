<!--suppress ALL -->
<div style="display: flex; gap: 5px">
  <Button
    kind="danger"
    size="small"
    icon={CheckmarkOutline32}
    on:click={handleClickMarkAsReadAll}>Mark all as read</Button>
  <Button
    size="small"
    icon={ProgressBarRound32}
    on:click={handleClickReloadConfig}>Reload config</Button>
  {#each results as r}
    {#if r.loading}
      <InlineLoading description={`Search ${r.item.condition.query}...`} />
    {/if}
  {/each}
</div>
<div style="padding-top: 20px;  height: calc(100vh - 100px - 50px);">
  {#if unreadResults.length === 0}
    <div
      transition:fade
      style="display: flex; justify-content: center; align-items: center; flex-direction: column; padding-top: 25vh">
      <img
        src="https://github.com/tadashi-aikawa/vigilancia/raw/master/src-tauri/icons/128x128@2x.png"
        alt="vigilancia" />
      <span style="font-size: 24px; font-weight: bolder; color: mediumpurple">
        There are no beneficial messages. No worries!
      </span>
    </div>
  {/if}

  <Tabs autoWidth>
    {#each unreadResults as r}
      <Tab>
        <div
          style="display: flex; align-items: center; height: 26px; padding-bottom: 10px;">
          <span style="margin-right: 3px;">
            {r.item.condition.title ?? r.item.condition.query}
          </span>
          {#if r.loading}
            <InlineLoading />
          {:else}
            <Tag type={r.item.condition.color ?? "cyan"} size="sm"
              >{unreadMessages(r.item.messages).length}</Tag>
          {/if}
        </div>
      </Tab>
    {/each}

    <svelte:fragment slot="content">
      {#each unreadResults as r, i}
        <TabContent>
          {#if r.error}
            <InlineNotification title="Error" subtitle={r.error} />
          {/if}
          <div style="margin: 0 0 10px 15px">
            {#each unreadMessages(r.item.messages) as message, i (message)}
              <span style="margin: 0 2px">
                <UserImage userId={message.user_id} size="24" />
              </span>
            {/each}

            <TooltipIcon
              size="small"
              style="cursor: pointer; margin-left: 30px;"
              tooltipText="Search by a current query"
              on:click={() => handleClickSearchByCurrentQuery(i)}>
              <Search24 style="fill: darkgreen" />
            </TooltipIcon>
            <TooltipIcon
              size="small"
              style="cursor: pointer; margin-left: 10px;"
              tooltipText="Mark messages in this tab as read"
              icon={CheckmarkOutline24}
              on:click={() => handleClickMarkAsReadItem(r.item)}>
              <CheckmarkOutline24 style="fill: orangered" />
            </TooltipIcon>
          </div>
          <div
            style=" height: calc(100vh - 100px - 50px - 100px); overflow-y: scroll">
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
    TooltipIcon,
    Button,
    InlineNotification,
    Tab,
    TabContent,
    Tabs,
    Tag,
    InlineLoading,
  } from "carbon-components-svelte";

  import { Message, Response } from "~/model/search-messages";
  import { Response as Config, Condition } from "~/model/fetch-config";
  import MessageCard from "~/components/molecules/MessageCard.svelte";
  import UserImage from "~/components/atoms/UserImage.svelte";
  import { sleep } from "~/utils/time";
  import {
    ProgressBarRound32,
    Search24,
    CheckmarkOutline24,
    CheckmarkOutline32,
  } from "carbon-icons-svelte";
  import { AsyncResult, DateTime, fromPromise, Nullable } from "owlelia";
  import { sendNotification } from "@tauri-apps/api/notification";
  import { onDestroy, onMount } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { flip } from "svelte/animate";

  type Item = {
    condition: Condition;
    messages: Message[];
    lastSearchDate?: DateTime;
  };
  type LiquidValue = {
    item: Item;
    loading: boolean;
    error: Nullable<string>;
  };

  let config: Config = {
    interval_sec: 60 * 10,
    conditions: [],
  };

  let readById: { [messageId: string]: DateTime } = {};
  let lastMessageIdByQuery: { [query: string]: Message["id"] } = {};
  let results: LiquidValue[] = [];

  $: unreadMessages = (messages: Message[]) =>
    messages.filter((x) => !readById[x.id]);
  $: unreadResults = results.filter(
    (x) => unreadMessages(x.item.messages).length > 0 || x.error
  );

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

  const searchItem = async (condition: Condition): Promise<Item> => {
    return invoke<Response>("search_messages", {
      query: `${condition.query} after:${
        DateTime.today().minusDays(1 + config.since_day_ago ?? 1).displayDate
      }`,
      excludeMe: !condition.include_me,
    }).then((r) => {
      return {
        condition,
        messages: r.messages,
        lastSearchDate: DateTime.now(),
      };
    });
  };

  const search = async (i: number, suppressNotify: boolean) => {
    results[i].loading = true;
    try {
      const condition = results[i].item.condition;
      const item = await searchItem(condition);
      results[i].item = item;

      const latestMessageId = item.messages?.[0]?.id;
      if (
        condition.should_notify &&
        !suppressNotify &&
        lastMessageIdByQuery[condition.query] !== latestMessageId &&
        !readById[latestMessageId]
      ) {
        sendNotification(
          `"${condition.query}" に関する新しいメッセージを見つけました🦋`
        );
      }
      lastMessageIdByQuery[condition.query] = latestMessageId;
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
    results = config.conditions.map((condition) => ({
      item: { condition, messages: [] },
      loading: false,
      error: null,
    }));
  };

  const handleClickReloadConfig = async () => {
    await invoke<void>("reload_config");
    await loadConfig();
    await subscribeSearchers();
  };

  const handleClickSearch = searchAll;

  let timeoutHandlers: number[] = [];
  const subscribeSearchers = async () => {
    timeoutHandlers.forEach((x) => window.clearTimeout(x));

    const endlessIntervalSearch = async (
      i: number,
      suppressNotify: boolean
    ) => {
      await search(i, suppressNotify);
      timeoutHandlers.push(
        window.setTimeout(() => {
          endlessIntervalSearch(i, false);
        }, (results[i].item.condition.interval_sec ?? config.interval_sec) * 1000)
      );
    };

    results.forEach((_, i) =>
      timeoutHandlers.push(
        window.setTimeout(() => {
          endlessIntervalSearch(i, true);
        }, 5 * 1000 * (i + 1))
      )
    );
  };

  onMount(async () => {
    timeoutHandlers.forEach((x) => window.clearTimeout(x));
    await loadConfig();
    await subscribeSearchers();
  });

  onDestroy(() => {
    timeoutHandlers.forEach((x) => window.clearTimeout(x));
  });
</script>

<style>
  .action-icon-wrapper {
    cursor: pointer;
    margin-right: 10px;
  }
  .action-icon-wrapper::hover {
    background-color: rgba(144, 188, 144, 0.4);
  }
</style>
