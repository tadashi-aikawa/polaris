<img
  src={user.images.image_512}
  alt={user.real_name}
  style={`border-radius: 5px; height: ${size}px;`} />

<script lang="ts">
  import type { User } from "~/model/fetch-all-users";
  import { userMap } from "~/stores";
  import type { Nullable } from "owlelia";

  export let user: Nullable<User>;
  export let userId: Nullable<string>;
  export let size: number = 48;

  $: user = user ? user : $userMap[userId];
</script>
