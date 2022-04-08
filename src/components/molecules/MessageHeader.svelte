{#if user}
  <div style="display: flex; gap: 10px;">
    <UserImage {user} size="48" />
    <div style="flex-direction: row; padding-top: 5px;">
      <div style="display: flex; align-items: center; gap: 7px;">
        <span style="font-weight: bold; font-size: 16px">{user.real_name}</span>
        <span style="font-size: 14px; color: dimgrey">
          {displayedCreatedAt}
        </span>
        <TooltipIcon
          tooltipText="Open in Slack"
          size="small"
          direction="right"
          icon={Launch16}
          style="cursor: pointer;"
          on:click={() => {
            shell.open(permalink);
          }} />
      </div>

      <div
        style="font-size: 16px; padding: 8px 0; color: dodgerblue; font-weight: bolder">
        #{channelName}
      </div>
    </div>
  </div>
{/if}

<script lang="ts">
  import { shell } from "@tauri-apps/api";
  import { TooltipIcon } from "carbon-components-svelte";
  import { Launch16 } from "carbon-icons-svelte";
  import { userMap } from "~/stores";
  import { DateTime } from "owlelia";
  import UserImage from "~/components/atoms/UserImage.svelte";

  export let userId: string;
  export let createdAt: string;
  export let channelName: string;
  export let permalink: string;

  $: displayedCreatedAt = DateTime.of(createdAt).displayDateTime;
  $: user = $userMap[userId];
</script>
