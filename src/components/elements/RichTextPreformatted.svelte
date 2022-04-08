<div class="pre">
  {#each element.elements as e}
    <MessageElement element={e} />
  {/each}
</div>

<script lang="ts">
  import MessageElement from "~/components/molecules/MessageElement.svelte";
  import type { RichTextPreformattedElement } from "~/model/search-messages";

  export let element: RichTextPreformattedElement;
</script>

<style>
  .pre {
    font-family: monospace;
    font-size: 85%;
    line-height: 16px;
    white-space: pre-wrap;
    word-wrap: anywhere;
    max-width: 95%;
    padding: 10px;
    margin-top: 5px;
    border-radius: 7px;
    border: solid 1px lightgrey;
    background-color: ghostwhite;
    vertical-align: baseline;
  }
</style>
