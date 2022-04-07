{#if attachment.message_blocks}
  {#each attachment.message_blocks ?? [] as mb}
    <div class="attachment">
      <div style="display: flex; align-items: center; gap: 5px;">
        <img
          src={attachment.author_icon}
          alt={attachment.author_name ?? attachment.author_subname}
          width="20" />
        <span style="font-size: 14px; font-weight: bolder">
          {attachment.author_name ?? attachment.author_subname}
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
          {#if attachment.from_url}
            <TooltipIcon
              tooltipText="Open in Slack"
              size="small"
              direction="right"
              icon={Launch16}
              style="cursor: pointer;"
              on:click={() => {
                shell.open(attachment.from_url);
              }} />
          {/if}
          #{attachment.channel_name} | {DateTime.of(Number(mb.ts))
            .displayDateTime}
        </div>
      {/each}
    </div>
  {/each}
{:else if attachment.image_url}
  <div
    class="cite"
    style="display: flex; flex-direction: column; gap: 5px; max-width: 640px;">
    <div style="display: flex; align-items: center; gap: 5px;">
      <img
        src={attachment.author_icon}
        alt={attachment.author_name ?? attachment.author_subname}
        width="20" />
      <span style="font-size: 14px; font-weight: bolder">
        {attachment.author_name}
      </span>
      <span style="font-size: 14px; color: grey">
        {attachment.author_subname}
      </span>
    </div>

    <div class="message-element-wrapper">
      <span>{attachment.text}</span>
    </div>

    <div
      style="color: dimgrey; font-size: 90%; display: flex; align-items: center; gap: 3px">
      <img src={attachment.footer_icon} alt={attachment.footer} width="20" />
      <span>
        {attachment.footer} | {DateTime.of(Number(attachment.ts))
          .displayDateTime}
      </span>
    </div>

    <img
      src={attachment.image_url}
      alt={attachment.image_url}
      style="max-width: 240px" />
  </div>
{/if}

<script lang="ts">
  import MessageElement from "~/components/molecules/MessageElement.svelte";
  import { TooltipIcon } from "carbon-components-svelte";
  import { Launch16 } from "carbon-icons-svelte";
  import { shell } from "@tauri-apps/api";
  import { Attachment } from "~/model/search-messages";
  import { DateTime } from "owlelia";

  export let attachment: Attachment;
</script>

<style>
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
  .cite {
    padding: 5px 20px 5px 20px;
    margin: 10px;
    border: 0 solid grey;
    border-left-width: 5px;
  }
</style>
