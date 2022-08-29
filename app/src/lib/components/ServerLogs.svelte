<script lang="ts">
  import type { ServerInfo } from "$lib/modals/api_types";
  import logStore from "$lib/stores/log";
import { time_ranges_to_array } from "svelte/internal";

  let old_server: ServerInfo | null = null;
  export let server: ServerInfo | null = null;

  let list: HTMLElement;

  $: if (old_server != server && server != null) {
    logs.send({
      type: "GetLogs",
      body: {
        id: server.id,
      },
    });
    old_server = server;
    setTimeout(scroll_to_bottom, 10);
  }

  const scroll_to_bottom = () => {
    list.scrollTop = list.scrollHeight*2;
  }

  $: if ($logs.pages.length > 0) {
    setTimeout(scroll_to_bottom, 10);
  }

  const logs = logStore("wss://localhost:18249", {
    pages: [],
  }, []);

  const format_ts = (ts:number) => {
    const d = new Date(ts*1000);
    return `${d.getHours()}:${d.getMinutes()}`;
  };
  const format_ts_full = (ts: number) => {
    const d = new Date(ts*1000);
    return d.toLocaleString();
  }
</script>

<article bind:this={list}>
  <ul>
    {#each $logs.pages as page}
      {#each page as msg}
        <li class="{msg.msg_type}">
          <span class="timestamp" title="{format_ts_full(msg.timestamp)}">{format_ts(msg.timestamp)}</span>
          <span class="body">{msg.body}</span>
        </li>
      {/each}  
    {/each}
  </ul>
</article>

<style lang="scss">
  article {
    box-sizing: border-box;

    background: rgba(0, 0, 0, 0.201);
    overflow-y: scroll;
    overflow-x: hidden;

    height: 100%;
    width: 100%;
  }
  ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  li {
    padding: 0.1em 0.5em;
    display: flex;
    &:first-child {
      padding-top: 0.2em;
    }
    &:hover {
      background: rgba(0, 0, 0, 0.201);
    }
  }

  .out {
    color: #FFFFFF7E;
  }

  .timestamp {
    opacity: 0.4;
    padding-right: 0.33em;
  }
</style>