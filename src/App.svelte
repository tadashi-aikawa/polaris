<main style="padding: 45px;">
  {#await initializePromise}
    <InlineLoading description="Initializing..." />
  {:then result}
    <EgoSearch />
  {:catch error}
    <InlineNotification title="Error" subtitle={error} />
  {/await}
</main>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  import { InlineLoading, InlineNotification } from "carbon-components-svelte";

  import EgoSearch from "~/components/organism/EgoSearch.svelte";
  import type { Response as EmojiResponse } from "~/model/fetch-emoji-list";
  import type { Response as AllUsersResponse } from "~/model/fetch-all-users";
  import { emojiMap, userMap } from "~/stores";

  let initializePromise: Promise<void> = Promise.resolve();

  onMount(async () => {
    initializePromise = new Promise<void>(async (resolve, reject) => {
      try {
        await invoke<void>("initialize");
        emojiMap.set((await invoke<EmojiResponse>("fetch_emoji_list")).emoji);
        userMap.set(
          (await invoke<AllUsersResponse>("fetch_all_users")).user_by_id
        );
        resolve();
      } catch (e) {
        reject(e);
      }
    });
  });
</script>
