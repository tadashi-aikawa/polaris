{#if user}
  <div style="display: flex; gap: 10px;">
    <UserImage {user} size="48" />
    <div style="flex-direction: row; padding-top: 5px;">
      <span style="font-weight: bold; font-size: 16px">{user.real_name}</span>
      <span style="font-size: 14px; margin-left: 5px; color: dimgrey">
        {displayedCreatedAt}
      </span>
      <div
        style="font-size: 16px; padding: 8px 0; color: dodgerblue; font-weight: bolder">
        #{channelName}
      </div>
    </div>
  </div>
{/if}

<script lang="ts">
  import { userMap } from "~/stores";
  import { DateTime } from "owlelia";
  import UserImage from "~/components/atoms/UserImage.svelte";

  export let userId: string;
  export let createdAt: string;
  export let channelName: string;

  $: displayedCreatedAt = DateTime.of(createdAt).displayDateTime;
  $: user = $userMap[userId];
</script>
