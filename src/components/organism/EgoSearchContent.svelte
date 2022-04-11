<div style="margin: 0 0 10px 15px">
  {#each messages as message, i (message)}
    <span style="margin: 0 2px">
      <UserImage userId={message.user_id} size="24" />
    </span>
  {/each}

  <TooltipIcon
    size="small"
    style="cursor: pointer; margin-left: 10px;"
    tooltipText="Mark messages in this tab as read"
    icon={CheckmarkOutline24}
    on:click={markAsReadMessages}>
    <CheckmarkOutline24 style="fill: orangered" />
  </TooltipIcon>
</div>
<div class="messages-wrapper">
  {#each messages as message, i (message)}
    <div
      style="padding: 5px;"
      animate:flip={{ duration: 500 }}
      in:fade
      out:fly={{ x: 100 }}>
      <MessageCard {message} on:click:read={handleClickMarkAsRead} />
    </div>
  {/each}
</div>

<script lang="ts">
  import { TooltipIcon } from "carbon-components-svelte";
  import UserImage from "~/components/atoms/UserImage.svelte";
  import { CheckmarkOutline24 } from "carbon-icons-svelte";
  import MessageCard from "~/components/molecules/MessageCard.svelte";
  import { Message } from "~/model/search-messages";
  import { fade, fly } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { DateTime } from "owlelia";
  import { readById } from "~/stores";
  import { sleep } from "~/utils/time";

  export let messages: Message[];

  const markAsRead = async (message: Message) => {
    readById.update((st) => ({ ...st, [message.id]: DateTime.now() }));
  };
  const markAsReadMessages = async () => {
    for (const m of messages) {
      await markAsRead(m);
      await sleep(200);
    }
  };

  const handleClickMarkAsRead = (event: CustomEvent<Message>) => {
    markAsRead(event.detail);
  };
</script>

<style>
  .messages-wrapper {
    height: calc(100vh - 100px - 35px - 100px);
    overflow-y: auto;
    overflow-x: hidden;
    padding: 0 30px 0 5px;
    max-width: 1080px;
  }
</style>
