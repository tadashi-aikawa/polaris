<ul class="ul" style="margin-left: {20 * element.indent}px">
  {#each element.elements as e}
    <li class="li">
      <MessageElement element={e} />
    </li>
  {/each}
</ul>

<script lang="ts">
  import MessageElement from "~/components/molecules/MessageElement.svelte";
  import type { RichTextListElement } from "~/model/search-messages";

  export let element: RichTextListElement;
</script>

<style>
  .ul {
    padding-left: 25px;
  }
  .li {
    list-style: disc;
    line-height: 22px;
  }
</style>
