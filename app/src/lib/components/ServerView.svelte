<script lang="ts">
  import type { ServerInfo } from "src/modals/api_types";
  import type State from "src/modals/State";
  import type { WSStore } from "src/stores/ws";
  import { getContext } from "svelte";
  export let server: ServerInfo | null = null;

  const ws: WSStore<State> = getContext("ws");

  const change_name = (name: string, el: HTMLInputElement) => {
    console.log("attmept to set name to", name)
    if(name === "") el.value = server?.name ?? "Select a server."; 
    if(!name || !server) return;
    console.log("setting name to", name);
    ws.send({
      type: "UpdateServer",
      body: {
        id: server.id,
        name,
      }
    })
  };
  const on_name_changed = (e: Event) => {
    const target = e.target as HTMLInputElement;
    change_name(target.value, target);
  }
  const on_name_key = (e: KeyboardEvent) => {
    if (e.key !== "Enter" || !e.currentTarget) return;
    const target = e.currentTarget as HTMLInputElement;
    change_name(target.value, target);
  }
</script>

<article>
  <header class:noserver="{server === null}">
    <input value={
        (server?.name ?? "Select a server.")
      }
      on:change="{on_name_changed}"
      on:keypress="{on_name_key}"
      disabled="{!server}"/>
    <button>Edit</button>
    <button>Power</button>
    <button>Open</button>
    <span class="grow"/>
    <button>Delete</button>
  </header>
  <section>

  </section>
</article>

<style lang="scss">
  @mixin minimal-input {
    margin: 0;
    padding: 0.1em 0.25em;

    color: rgba(136, 138, 139, 0.5);
    background: none;
    outline: 0;
    border: 0;

    transition: background-color 0.1s ease-in-out,
    color 0.1s ease-in-out;
    
    &:enabled {
      cursor: pointer;
    }
    &:enabled:hover {
      background: rgba(0,0,0,0.2);
    }

    &:focus {
      color: #888A8B;
      background: rgba(0,0,0,0.3);
      &::placeholder {
        opacity: 0;
      }
    }
  }
  button {
    border: none;
    outline: none;

    background: none;
    color: white;

    opacity: 0.45;

    text-transform: uppercase;
    cursor: pointer;

    transition: opacity 0.1s ease-in-out;

    &:hover {
      opacity: 0.8;
    }
  }

  .grow {
    flex-grow: 1;
  }

  header {
    display: flex;
    margin: 1em 1em 1em 1em;
    align-items: baseline;
    &>input {
      @include minimal-input;
      margin: 0 .5em 0 0;

      width: min-content;

      height: 2em;

      font-size: 1.5em;

      background: none;
      outline: none;
      border: none;
      color: #EEEEEE;
    }
    * {
      padding: 0.5em 0.8em;
    }
    border-bottom: 1px solid #2E3338;

    &.noserver {
      button {
        visibility: hidden;
      }
    }
  }
</style>