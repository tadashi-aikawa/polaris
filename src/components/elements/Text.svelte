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
  .bold {
    font-weight: bold;
  }
  .code {
    color: #c7254e;
    background-color: #eee8d5;
    border: solid 1px lightgrey;
    font-size: 90%;
    padding: 2px;
    margin: 2px;
    border-radius: 5px;
    vertical-align: baseline;
  }
</style>
