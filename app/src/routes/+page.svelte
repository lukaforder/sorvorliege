<script lang="ts">
  import ServerView from "../lib/components/ServerView.svelte";
  import { setContext } from "svelte";

  import ServerList from "../lib/components/ServerList.svelte";
  import {wsStore} from "../stores/ws/ws";

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

<ServerList bind:selected_server={current_server}/>
<ServerView server={current_server ? $ws.servers[current_server] : null} />

<style lang="scss">
  :global(body) {
    display: grid;
    grid-template-columns: 1fr 1fr;
  }
</style>