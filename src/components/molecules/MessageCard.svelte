<Tile style="display: flex; flex-direction: column; gap: 15px;">
  <div style="display: flex; gap: 10px; align-items: center">
    <span style="font-weight: bold;">{message.user_name}</span>
    <span style="font-size: 80%;">{message.created_at}</span>
    <small>#{message.channel_name}</small>
  </div>
  <pre style="white-space: pre-wrap">{message.text}</pre>

  <div style="display: flex; gap: 15px;">
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
