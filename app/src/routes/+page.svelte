<script lang="ts">
  import ServerView from "../lib/components/ServerView.svelte";
  import { setContext } from "svelte";

  import ServerList from "../lib/components/ServerList.svelte";
  import {wsStore} from "../lib/stores/ws/ws";
import ServerLogs from "$lib/components/ServerLogs.svelte";

  export const ws = wsStore("wss://localhost:18249", {
    isConnected: false,
    servers: {},
    testVar: 0
  }, []);
  setContext("ws", ws);

  let current_server: string | null = null;
</script>

<svelte:head>
  <link rel="stylesheet" href="/style/dark.css">
</svelte:head>

<section class="list">
  <ServerList bind:selected_server={current_server}/>
</section>
<section class="view">
  <ServerView server={current_server ? $ws.servers[current_server] : null} />
</section>
<section class="logs">
  <ServerLogs server={current_server ? $ws.servers[current_server] : null} />
</section>

<style lang="scss">
  :global(body) {
    display: grid;
    grid-template-columns: 50vw 50vw;
    grid-template-rows: 50vh 50vh;
    grid-template-areas: "list view" "list logs";

    width: 100vw;
    height: 100vh;
    box-sizing: border-box;

    margin: 0;
    padding: 0;
  }

  section {
    padding: 0.5em;
  }

  .list {
    grid-area: list;
  }
  .view {
    grid-area: view;
  }
  .logs {
    grid-area: logs;
  }
</style>