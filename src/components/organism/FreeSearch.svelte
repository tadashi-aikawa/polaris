<div style="display: flex; gap: 20px">
  <TextInput bind:value={query} placeHolder="search query" />
  <Button size="small" icon={Search32} on:click={handleClickSearch}
    >Search</Button>
</div>
<div
  style="padding: 30px; margin-top: 15px; height: calc(100vh - 100px - 50px); overflow-y: scroll">
  {#await messagesPromise}
    <InlineLoading description="Loading..." />
  {:then messages}
    {#each messages as message}
      <div style="padding: 5px;">
        <MessageCard {message} />
      </div>
    {/each}
  {:catch error}
    <InlineNotification title="Error" subtitle={error} />
  {/await}
</div>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    Button,
    InlineLoading,
    InlineNotification,
    TextInput,
  } from "carbon-components-svelte";

  import { Message, Response } from "~/model/search-messages";
  import MessageCard from "~/components/molecules/MessageCard.svelte";
  import { Search32 } from "carbon-icons-svelte";

  let messagesPromise: Promise<Message[]> = Promise.resolve([]);
  let query = "";

  const handleClickSearch = () => {
    messagesPromise = invoke<Response>("search_messages", { query }).then(
      (r) => r.messages
    );
  };
</script>
