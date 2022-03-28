<Card>
  <Content>
    <div style="display: flex; gap: 10px; align-items: center">
      <b>{message.user_name}</b>
      <span style="font-size: 80%;">{message.created_at}</span>
      <small>#{message.channel_name}</small>
    </div>
    <pre style="white-space: pre-wrap">{message.text}</pre>
  </Content>
  <Actions>
    <IconButton
      class="material-icons"
      on:click={() => {
        window.open(message.permalink);
      }}
      title="Share">open_in_new</IconButton>
  </Actions>
</Card>

<script lang="ts">
  import Card, { Content, Actions } from "@smui/card";
  import IconButton from "@smui/icon-button";
  import type { Message } from "~/model/search-messages";

  export let message: Message;
</script>
