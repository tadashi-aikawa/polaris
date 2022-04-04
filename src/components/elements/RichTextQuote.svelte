<div class="cite">
  {#each element.elements as e}
    <MessageElement element={e} />
  {/each}
</div>

<script lang="ts">
  import MessageElement from "~/components/molecules/MessageElement.svelte";
  import type { RichTextQuoteElement } from "~/model/search-messages";

  export let element: RichTextQuoteElement;
</script>

<style>
  .cite {
    padding: 5px 20px 5px 20px;
    margin: 10px;
    border: 0 solid grey;
    border-left-width: 5px;
  }
</style>
