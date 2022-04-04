{#each element.elements as e}
  <MessageElement element={e} />
{/each}

<script lang="ts">
  import MessageElement from "~/components/molecules/MessageElement.svelte";
  import type { RichTextSectionElement } from "~/model/search-messages";

  export let element: RichTextSectionElement;
</script>
