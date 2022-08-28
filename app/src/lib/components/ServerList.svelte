<script lang="ts">
  import type State from "src/modals/State";
  import type { WSStore } from "src/stores/ws";
  import { createEventDispatcher, getContext } from "svelte";

  export let selected_server: string | null = null;
  const ws: WSStore<State> = getContext("ws");

  const create_server = () => {
    ws?.send({
      type: "CreateServer",
    });
  }
</script>

<ul>
  {#each Object.values($ws?.servers) as server }
    <li role="button"
        class:current="{selected_server===server.id}"
        on:click="{() => selected_server = server.id}"
      >
      <h1>{server.name}</h1>
      <h2>{server.comm_status}</h2>
    </li>
  {/each}
  <li role="button" on:click="{create_server}">+</li>
</ul>

<style lang="scss">
  $SERVER_HEIGHT: 54px;
  ul {
    margin: 0;
    padding: 0;

    display: grid;
    grid-template-columns: 2fr auto auto;

    height: fit-content;
    user-select: none;

    justify-content: center;
    align-items: center;

    overflow-y: auto;

    li {
      display: grid;
      grid-column: 1/4;
      grid-template-columns: subgrid; // FIXME: stop using subgrid

      align-items: center;

      height: $SERVER_HEIGHT;

      border-bottom: 1px solid #2E3338;

      cursor: pointer;

      h1,h2 {
        padding: 8px 1em;
        text-align: right;
        opacity: .5;
        color: #eeeeee;
      }

      h1 {
        opacity: 1;
        text-align: left;
        font-size: 1rem;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
      }

      h2 {
        font-size: 0.9rem;
      }

      &:last-child {
        grid-template-columns: 1fr;
        text-align: center;
      }

      &.current, &:hover {
        background: #2E3338;
      }
    }
  }
</style>