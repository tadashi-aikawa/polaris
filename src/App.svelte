<main style="padding: 45px;">
  {#await initializePromise}
    <InlineLoading description={loadingText} />
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
  import type { Response as AllUserGroupsResponse } from "~/model/fetch-all-usergroups";
  import { emojiMap, userGroupMap, userMap } from "~/stores";

  let initializePromise: Promise<void> = Promise.resolve();
  let loadingText = "";

  onMount(async () => {
    initializePromise = new Promise<void>(async (resolve, reject) => {
      try {
        loadingText = "Initializing start";
        await invoke<void>("initialize");

        loadingText = "Load emoji list";
        emojiMap.set((await invoke<EmojiResponse>("fetch_emoji_list")).emoji);

        loadingText = "Load user list";
        userMap.set(
          (await invoke<AllUsersResponse>("fetch_all_users")).user_by_id
        );

        loadingText = "Load user group list";
        userGroupMap.set(
          (await invoke<AllUserGroupsResponse>("fetch_all_usergroups"))
            .usergroup_by_id
        );

        loadingText = "Finish";
        resolve();
      } catch (e) {
        reject(e);
      }
    });
  });
</script>
