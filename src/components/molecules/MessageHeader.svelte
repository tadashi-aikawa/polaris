<div style="display: flex; gap: 10px;">
  <img
    src={user.images.image_512}
    alt={user.real_name}
    style="border-radius: 5px; height: 48px;" />
  <div style="flex-direction: row">
    <span style="font-weight: bold; font-size: 16px">{user.real_name}</span>
    <span style="font-size: 14px; margin-left: 5px; color: dimgrey">
      {createdAt}
    </span>
    <div
      style="font-size: 18px; padding: 8px 0; color: dodgerblue; font-weight: bolder">
      #{channelName}
    </div>
  </div>
</div>

<script lang="ts">
  import { userMap } from "~/stores";

  export let userId: string;
  export let createdAt: string;
  export let channelName: string;

  $: user = $userMap[userId];
</script>
