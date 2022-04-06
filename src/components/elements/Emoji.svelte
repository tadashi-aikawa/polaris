{#if emojiUrl}
  {#if emojiUrl.startsWith("alias:")}
    <img
      src={$emojiMap[emojiUrl.replace("alias:", "")]}
      style="vertical-align: middle"
      width="24px"
      height="24px"
      alt={$emojiMap[emojiUrl.replace("alias:", "")]} />
  {:else}
    <img
      src={emojiUrl}
      style="vertical-align: middle"
      width="24px"
      height="24px"
      alt={emojiUrl} />
  {/if}
{:else}
  <span style="font-size: 18px;">{emoji.get(emojiName)}</span>
{/if}

<script lang="ts">
  import { emojiMap } from "~/stores";
  import * as emoji from "node-emoji";
  import type { EmojiElement } from "~/model/search-messages";

  export let element: EmojiElement;

  $: emojiName = element.name;
  $: emojiUrl = $emojiMap[emojiName];
</script>
