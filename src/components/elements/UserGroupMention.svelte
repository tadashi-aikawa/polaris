<span class="user-group">
  @{userGroup.name}
</span>

<script lang="ts">
  import { userGroupMap } from "~/stores";
  import type { UserGroupElement } from "~/model/search-messages";

  export let element: UserGroupElement;

  $: userGroup = $userGroupMap[element.usergroup_id];
</script>

<style>
  .user-group {
    color: dodgerblue;
    background-color: powderblue;
    margin-right: 3px;
    line-height: 30px;
  }
</style>
