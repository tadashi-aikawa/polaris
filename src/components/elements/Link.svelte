<Link target="_blank" style="font-size: 15px" href={element.url}>
  {element.text ?? element.url}
</Link>

<script lang="ts">
  import type { LinkElement } from "~/model/search-messages";
  import { Link } from "carbon-components-svelte";

  export let element: LinkElement;
</script>
