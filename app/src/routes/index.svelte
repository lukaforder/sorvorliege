<script>

  import {wsStore} from "../stores/ws/ws";

  export const ws = wsStore("wss://localhost:18249", {
    isConnected: false,
    testVar: 0
  }, []);

  let inc = 1;
</script>

<svelte:head>
  <link rel="stylesheet" href="/style/dark.css">
</svelte:head>

<ul>
  <li><h1>Server 1</h1><h2>Active</h2></li>
  <li><h1>Server 2</h1><h2>Inactive</h2></li>
  <li><h1>Server 3</h1><h2>Inactive</h2></li>
  <li><h1>Server 4</h1><h2>Inactive</h2></li>
  <li><h1>Server 5</h1><h2>Inactive</h2></li>
</ul>

<style>
  ul {
    list-style: none;
    padding: 0;

    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-auto-flow: row;

    font-size: 18px;
  }

  ul li {
    display: contents;
  }

  ul li * {
    margin: 0;
    padding: 0;

    display: flex;
    align-items: center;

    font-size: 1em;
    font-weight: 400;
    opacity: 0.6;
  }

  ul h1 {
    font-size: 1em;
    opacity: 1;
  }
</style>