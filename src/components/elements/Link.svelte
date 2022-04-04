<Link target="_blank" class="link" href={element.url}>
  {element.text ?? element.url}
</Link>

<script lang="ts">
  import type { LinkElement } from "~/model/search-messages";
  import { Link } from "carbon-components-svelte";

  export let element: LinkElement;
</script>

<style>
  .link {
    font-size: 15px;
  }
</style>
