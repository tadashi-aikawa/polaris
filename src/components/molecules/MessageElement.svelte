{#if element.type === "rich_text_section"}
  {#each element.elements as e}
    <MessageElement element={e} />
  {/each}
{:else if element.type === "rich_text_quote"}
  <div class="cite">
    {#each element.elements as e}
      <MessageElement element={e} />
    {/each}
  </div>
{:else if element.type === "rich_text_list"}
  <ul class="ul" style="margin-left: {20 * element.indent}px">
    {#each element.elements as e}
      <li class="li">
        <MessageElement element={e} />
      </li>
    {/each}
  </ul>
{:else if element.type === "rich_text_preformatted"}
  <div class="pre">
    {#each element.elements as e}
      <MessageElement element={e} />
    {/each}
  </div>
{:else if element.type === "emoji"}
  💩
  <!--    TODO: 絵文字を表示-->
{:else if element.type === "text"}
  <span
    class="text"
    class:bold={element.style?.bold}
    class:code={element.style?.code}>
    {element.text}
  </span>
{:else if element.type === "link"}
  <Link target="_blank" class="link" href={element.url}
    >{element.text ?? element.url}</Link>
{:else if element.type === "user"}
  <span class="user">
    @{element.user_id}
  </span>
  <!--    TODO: ユーザー名を表示-->
{:else if element.type === "broadcast"}
  <span
    class="broadcast"
    class:bold={element.style?.bold}
    class:code={element.style?.code}>
    @{element.range}
  </span>
{:else if element.type === "channel"}
  <span
    class="channel"
    class:bold={element.style?.bold}
    class:code={element.style?.code}>
    #{element.channel_id}
    <!--    TODO: channel名を表示-->
  </span>
{:else}
  <li>{element.type}</li>
{/if}

<script lang="ts">
  import type { Element } from "~/model/search-messages";
  import MessageElement from "~/components/molecules/MessageElement.svelte";
  import { Link } from "carbon-components-svelte";
  export let element: Element;
</script>

<style>
  .text {
    font-size: inherit;
    line-height: inherit;
  }
  .bold {
    font-weight: bold;
  }
  .pre {
    font-family: monospace;
    font-size: 85%;
    line-height: 16px;
    white-space: pre-wrap;
    padding: 10px;
    margin-top: 5px;
    border-radius: 7px;
    border: solid 1px lightgrey;
    background-color: ghostwhite;
    vertical-align: baseline;
  }
  .code {
    color: #c7254e;
    background-color: #eee8d5;
    font-size: 90%;
    padding: 2px;
    margin: 2px;
    border-radius: 5px;
    vertical-align: baseline;
  }
  .cite {
    padding: 5px 20px 5px 20px;
    margin: 10px;
    border: 0 solid grey;
    border-left-width: 5px;
  }
  .link {
    font-size: 15px;
  }
  .user {
    color: dodgerblue;
    background-color: powderblue;
    margin-right: 3px;
  }
  .channel {
    color: dodgerblue;
    background-color: powderblue;
    margin-right: 3px;
  }
  .broadcast {
    font-weight: bold;
    background-color: #ffdb79;
    margin-right: 3px;
  }

  .ul {
    padding-left: 25px;
  }
  .li {
    list-style: disc;
    line-height: 22px;
  }
</style>
