<Tile style="padding: 0">
  <div style="display: flex; flex-direction: row;">
    <div class="read-button" on:click={handleClickRead}>
      <CheckmarkOutline24 size="small" style="cursor: inherit" />
    </div>

    <div style="padding: 15px 15px 15px 0;">
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

        {#each message.attachments ?? [] as ma}
          <div style="margin: 5px 0">
            <MessageAttachment attachment={ma} />
          </div>
        {/each}

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
    </div>
  </div>
</Tile>

<script lang="ts">
  import { shell } from "@tauri-apps/api";
  import { Tile, TooltipIcon } from "carbon-components-svelte";
  import { Launch20, Launch16, CheckmarkOutline24 } from "carbon-icons-svelte";

  import type { Message } from "~/model/search-messages";
  import { createEventDispatcher } from "svelte";
  import MessageElement from "~/components/molecules/MessageElement.svelte";
  import MessageHeader from "~/components/molecules/MessageHeader.svelte";
  import { DateTime } from "owlelia";
  import MessageAttachment from "~/components/molecules/MessageAttachment.svelte";

  export let message: Message;

  const dispatch = createEventDispatcher();
  const handleClickRead = () => {
    dispatch("click:read", message);
  };
</script>

<style>
  .vertical-space {
    display: flex;
    flex-direction: row;
  }
  .read-button {
    display: flex;
    align-items: center;
    cursor: pointer;
    padding: 0 10px;
    margin-right: 5px;
    color: darkgrey;
  }
  .read-button:hover {
    background-color: rgba(144, 188, 144, 0.4);
  }
  .message-element-wrapper {
    white-space: pre-wrap;
    line-height: normal;
    padding: 10px 0;
  }
</style>
