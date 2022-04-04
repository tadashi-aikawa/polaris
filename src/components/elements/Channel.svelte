<span
  class="channel"
  class:bold={element.style?.bold}
  class:code={element.style?.code}>
  #{element.channel_id}
  <!--    TODO: channel名を表示-->
</span>

<script lang="ts">
  import type { ChannelElement } from "~/model/search-messages";

  export let element: ChannelElement;
</script>

<style>
  .channel {
    color: dodgerblue;
    background-color: powderblue;
    margin-right: 3px;
  }
  .bold {
    font-weight: bold;
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
