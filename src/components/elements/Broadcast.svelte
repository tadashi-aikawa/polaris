<span
  class="broadcast"
  class:bold={element.style?.bold}
  class:code={element.style?.code}
  class:strike={element.style?.strike}>
  @{element.range}>
</span>

<script lang="ts">
  import type { BroadcastElement } from "~/model/search-messages";

  export let element: BroadcastElement;
</script>

<style>
  .broadcast {
    font-weight: bold;
    background-color: #ffdb79;
    margin-right: 3px;
  }
  .bold {
    font-weight: bold;
  }
  .strike {
    text-decoration: line-through;
  }
  .code {
    color: #c7254e;
    background-color: #eee8d5;
    font-size: 90%;
    padding: 2px;
    margin: 2px;
    border-radius: 5px;
    vertical-align: baseline;
  }
</style>
