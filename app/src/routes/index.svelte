<script>
  import {wsStore} from "../stores/ws";

  export const ws = wsStore("wss://localhost:18249", {
    isConnected: false,
    testVar: 0
  }, []);
</script>

<p> {$ws} </p>