<Tile style="padding: 0">
  <div class="vertical-space">
    <div class="main">
      <div style="display: flex; flex-direction: column; gap: 15px;">
        <MessageHeader
          userId={message.user_id}
          createdAt={message.created_at}
          channelName={message.channel_name} />

        <div class="message-element-wrapper">
          {#if message.blocks}
            {#each message.blocks as block}
              {#each block.elements ?? [] as element}
                <MessageElement {element} />
              {/each}
            {/each}
          {:else}
            <span>{message.text}</span>
          {/if}
        </div>

        {#each message.attachments ?? [] as ma}
          {#each ma.message_blocks ?? [] as mb}
            <div class="attachment">
              <div style="display: flex; align-items: center; gap: 5px;">
                <img
                  src={ma.author_icon}
                  alt={ma.author_name ?? ma.author_subname}
                  width="20" />
                <span style="font-size: 14px; font-weight: bolder">
                  {ma.author_name ?? ma.author_subname}
                </span>
              </div>

              {#each mb.message.blocks as b}
                <div class="message-element-wrapper">
                  {#each b.elements ?? [] as element}
                    <MessageElement {element} />
                  {/each}
                </div>
                <div
                  style="color: dimgrey; font-size: 90%; display: flex; align-items: center; gap: 3px">
                  {#if ma.from_url}
                    <TooltipIcon
                      tooltipText="Open in Slack"
                      size="small"
                      direction="right"
                      icon={Launch16}
                      style="cursor: pointer;"
                      on:click={() => {
                        shell.open(ma.from_url);
                      }} />
                  {/if}
                  #{ma.channel_name} | {DateTime.of(Number(mb.ts))
                    .displayDateTime}
                </div>
              {/each}
            </div>
          {/each}
        {/each}
      </div>

      <div style="display: flex; gap: 15px;">
        <TooltipIcon
          tooltipText="Open in Slack"
          size="small"
          direction="right"
          icon={Launch20}
          style="cursor: pointer"
          on:click={() => {
            shell.open(message.permalink);
          }} />
      </div>
    </div>

    <div class="read-button" on:click={handleClickRead}>
      <CheckmarkOutline32 size="small" style="cursor: inherit" />
    </div>
  </div>
</Tile>

<script lang="ts">
  import { shell } from "@tauri-apps/api";
  import { Tile, TooltipIcon } from "carbon-components-svelte";
  import { Launch20, Launch16, CheckmarkOutline32 } from "carbon-icons-svelte";

  import type { Message } from "~/model/search-messages";
  import { createEventDispatcher } from "svelte";
  import MessageElement from "~/components/molecules/MessageElement.svelte";
  import MessageHeader from "~/components/molecules/MessageHeader.svelte";
  import { DateTime } from "owlelia";

  export let message: Message;

  const dispatch = createEventDispatcher();
  const handleClickRead = () => {
    dispatch("click:read", message);
  };
</script>

<style>
  .main {
    display: flex;
    flex-direction: column;
    padding: 15px;
    gap: 15px;
  }
  .vertical-space {
    display: flex;
    flex-direction: row;
    gap: 15px;
  }
  .read-button {
    display: flex;
    align-items: center;
    cursor: pointer;
    margin-left: auto;
    padding: 5px 15px;
  }
  .read-button:hover {
    background-color: rgba(144, 188, 144, 0.4);
  }
  .attachment {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 10px;
    border: solid 2px lightgrey;
    border-radius: 10px;
  }
  .message-element-wrapper {
    white-space: pre-wrap;
    line-height: normal;
  }
</style>
