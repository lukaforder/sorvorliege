<script>
  import {wsStore} from "../stores/ws/ws";

  export const ws = wsStore("wss://localhost:18249", {
    isConnected: false,
    testVar: 0
  }, []);

  let inc = 1;
</script>

<p> {JSON.stringify($ws)} </p>
<input type="number" bind:value={inc} min=0 />
<button on:click={() => {
  ws.increment(inc);
}}>Increment</button>