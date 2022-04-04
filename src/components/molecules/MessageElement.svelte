{#if element.type === "rich_text_section"}
  <RichTextSection {element} />
{:else if element.type === "rich_text_quote"}
  <RichTextQuote {element} />
{:else if element.type === "rich_text_list"}
  <RichTextList {element} />
{:else if element.type === "rich_text_preformatted"}
  <RichTextPreformatted {element} />
{:else if element.type === "emoji"}
  <Emoji {element} />
{:else if element.type === "text"}
  <Text {element} />
{:else if element.type === "link"}
  <Link {element} />
{:else if element.type === "user"}
  <UserMention {element} />
{:else if element.type === "usergroup"}
  <UserGroupMention {element} />
{:else if element.type === "broadcast"}
  <Broadcast {element} />
{:else if element.type === "channel"}
  <Channel {element} />
{:else}
  <li>{element.type}</li>
{/if}

<script lang="ts">
  import type { Element } from "~/model/search-messages";
  import Emoji from "~/components/elements/Emoji.svelte";
  import UserMention from "~/components/elements/UserMention.svelte";
  import UserGroupMention from "~/components/elements/UserGroupMention.svelte";
  import RichTextSection from "~/components/elements/RichTextSection.svelte";
  import RichTextQuote from "~/components/elements/RichTextQuote.svelte";
  import RichTextList from "~/components/elements/RichTextList.svelte";
  import RichTextPreformatted from "~/components/elements/RichTextPreformatted.svelte";
  import Text from "~/components/elements/Text.svelte";
  import Link from "~/components/elements/Link.svelte";
  import Broadcast from "~/components/elements/Broadcast.svelte";
  import Channel from "~/components/elements/Channel.svelte";

  export let element: Element;
</script>
