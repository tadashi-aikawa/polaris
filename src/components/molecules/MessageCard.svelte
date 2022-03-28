<Tile>
  <div style="display: flex; gap: 10px; align-items: center">
    <b>{message.user_name}</b>
    <span style="font-size: 80%;">{message.created_at}</span>
    <small>#{message.channel_name}</small>
  </div>
  <pre style="white-space: pre-wrap">{message.text}</pre>

  <div style="display: flex; gap: 15px; padding-top: 15px">
    <TooltipIcon
      tooltipText="Open in Slack"
      size="small"
      icon={Launch20}
      style="cursor: pointer"
      on:click={() => {
        window.open(message.permalink);
      }} />
  </div>
</Tile>

<script lang="ts">
  import { Tile, TooltipIcon } from "carbon-components-svelte";
  import { Button } from "carbon-components-svelte";
  import Launch20 from "carbon-icons-svelte/lib/Launch20";

  import type { Message } from "~/model/search-messages";

  export let message: Message;
</script>
