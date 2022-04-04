<span
  class="text"
  class:bold={element.style?.bold}
  class:code={element.style?.code}>
  {element.text}
</span>

<script lang="ts">
  import type { TextElement } from "~/model/search-messages";

  export let element: TextElement;
</script>

<style>
  .text {
    font-size: inherit;
    line-height: inherit;
  }
</style>
