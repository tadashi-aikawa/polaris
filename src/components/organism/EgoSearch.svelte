<div style="display: flex; justify-content: end">
  {#each results as r}
    {#if r.loading}
      <InlineLoading description={`Search ${r.item.condition.query}...`} />
    {/if}
  {/each}
  <Button
    size="small"
    icon={ProgressBarRound32}
    on:click={handleClickReloadConfig}>Reload config</Button>
</div>
<div style="padding-top: 5px;  height: calc(100vh - 100px - 35px);">
  {#if unreadResults.length === 0}
    <div
      style="display: flex; justify-content: center; align-items: center; flex-direction: column; padding-top: 25vh">
      <img
        src="https://github.com/tadashi-aikawa/vigilancia/raw/master/src-tauri/icons/128x128@2x.png"
        alt="vigilancia" />
      <span style="font-size: 24px; font-weight: bolder; color: mediumpurple">
        There are no beneficial messages. No worries!
      </span>
    </div>
  {:else}
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
          <EgoSearchContent
            messages={unreadMessages(r.item.messages)}
            error={r.error} />
        {/each}
      </svelte:fragment>
    </Tabs>
  {/if}
</div>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    Button,
    Tab,
    Tabs,
    Tag,
    InlineLoading,
  } from "carbon-components-svelte";

  import { Message, Response } from "~/model/search-messages";
  import { Response as Config, Condition } from "~/model/fetch-config";
  import { ProgressBarRound32 } from "carbon-icons-svelte";
  import { DateTime, Nullable } from "owlelia";
  import { sendNotification } from "@tauri-apps/api/notification";
  import { onDestroy, onMount } from "svelte";
  import type { EgoSearchItem, EgoSearchLiquidValue } from "~/model/EgoSearch";
  import EgoSearchContent from "~/components/organism/EgoSearchContent.svelte";
  import { readById } from "~/stores";

  let config: Config = {
    interval_sec: 60 * 10,
    conditions: [],
    since_day_ago: null,
  };

  let lastMessageIdByQuery: { [query: string]: Nullable<Message["id"]> } = {};
  let results: EgoSearchLiquidValue[] = [];

  $: unreadMessages = (messages: Message[]) =>
    messages.filter((x) => !$readById[x.id]);
  $: unreadResults = results.filter(
    (x) => unreadMessages(x.item.messages).length > 0 || x.error
  );

  const searchItem = async (condition: Condition): Promise<EgoSearchItem> => {
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
      // noinspection OverlyComplexBooleanExpressionJS
      if (
        condition.should_notify &&
        !suppressNotify &&
        lastMessageIdByQuery[condition.query] !== latestMessageId &&
        !$readById[latestMessageId]
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
