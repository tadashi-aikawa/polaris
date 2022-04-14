{#if user}
  <span class="user">
    @{user.name}
  </span>
{/if}

<script lang="ts">
  import { userMap } from "~/stores";
  import type { UserElement } from "~/model/search-messages";

  export let element: UserElement;

  $: user = userMap.getUserById(element.user_id);
</script>

<style>
  .user {
    color: dodgerblue;
    background-color: powderblue;
    margin-right: 3px;
    line-height: 30px;
  }
</style>
