<main>
  <TextField
    bind:value={query}
    label="search query"
    style="min-width: 250px; margin-left: 30px" />
  <Button on:click={handleClickSearch}>
    <Icon class="material-icons">search</Icon>
    <Label>Search</Label>
  </Button>
  <div style="padding: 30px">
    {#await messagesPromise}
      <LinearProgress indeterminate />
    {:then messages}
      {#each messages as message}
        <div style="padding: 5px;">
          <MessageCard {message} />
        </div>
      {/each}
    {:catch error}
      <Paper variant="outlined" style="color: red; border-color: red;">
        <Title>Error</Title>
        <Content>{error}</Content>
      </Paper>
    {/await}
  </div>
</main>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import Button, { Label } from "@smui/button";
  import { Icon } from "@smui/icon-button";
  import Paper, { Title, Content } from "@smui/paper";
  import TextField from "@smui/textfield";
  import LinearProgress from "@smui/linear-progress";

  import { Message, Response } from "~/model/search-messages";
  import MessageCard from "~/components/molecules/MessageCard.svelte";

  let messagesPromise: Promise<Message[]> = Promise.resolve([]);
  let query = "";

  const handleClickSearch = () => {
    messagesPromise = invoke<Response>("search_messages", { query }).then(
      (r) => r.messages
    );
  };
</script>
