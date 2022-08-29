<script lang="ts">
  import type { ServerInfo } from "$lib/modals/api_types";
  import logStore from "$lib/stores/log";

  let old_server: ServerInfo | null = null;
  export let server: ServerInfo | null = null;

  $: if (old_server != server && server != null) {
    logs.send({
      type: "GetLogs",
      body: {
        id: server.id,
      },
    });
    old_server = server;
  }
  const logs = logStore("wss://localhost:18249", {
    pages: [],
  }, []);
</script>

<ul>
  {#each $logs.pages as page}
    {#each page as msg}
      <li>{msg.body}</li>
    {/each}  
  {/each}
</ul>

<style lang="scss">

</style>