<Tile style="padding: 0">
  <div style="display: flex; flex-direction: row;">
    <div class="read-button" on:click={handleClickRead}>
      <CheckmarkOutline24 size="small" style="cursor: inherit" />
    </div>

    <div style="padding: 15px 30px 0 0;">
      <MessageHeader
        userId={message.user_id}
        createdAt={message.created_at}
        channelName={message.channel_name}
        permalink={message.permalink} />

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
          <div style="max-width: 90%;">
            <MessageAttachment attachment={ma} />
          </div>
        {/each}
      </div>
    </div>
  </div>
</Tile>

<script lang="ts">
  import { Tile } from "carbon-components-svelte";
  import { CheckmarkOutline24 } from "carbon-icons-svelte";

  import type { Message } from "~/model/search-messages";
  import { createEventDispatcher } from "svelte";
  import MessageElement from "~/components/molecules/MessageElement.svelte";
  import MessageHeader from "~/components/molecules/MessageHeader.svelte";
  import MessageAttachment from "~/components/molecules/MessageAttachment.svelte";

  export let message: Message;

  const dispatch = createEventDispatcher();
  const handleClickRead = () => {
    dispatch("click:read", message);
  };
</script>

<style>
  .read-button {
    display: flex;
    align-items: center;
    cursor: pointer;
    padding: 0 10px;
    margin-right: 15px;
    color: darkgrey;
    border-right: dashed 1px lightgrey;
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
