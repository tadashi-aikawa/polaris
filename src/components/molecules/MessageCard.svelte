<Tile style="padding: 0">
  <div class="vertical-space">
    <div class="main">
      <div style="display: flex; flex-direction: column; gap: 15px;">
        <MessageHeader
          userId={message.user_id}
          createdAt={message.created_at}
          channelName={message.channel_name} />
        <div style="white-space: pre-wrap; line-height: normal;">
          {#if message.blocks}
            {#each message.blocks as block}
              {#each block.elements as element}
                <MessageElement {element} />
              {/each}
            {/each}
          {:else}
            <span>{message.text}</span>
          {/if}
        </div>
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
  import { Launch20, CheckmarkOutline32 } from "carbon-icons-svelte";

  import type { Message } from "~/model/search-messages";
  import { createEventDispatcher } from "svelte";
  import MessageElement from "~/components/molecules/MessageElement.svelte";
  import MessageHeader from "~/components/molecules/MessageHeader.svelte";

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
</style>
