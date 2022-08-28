<script lang="ts">
  import { CommunicatorType, type ServerInfo } from "../../modals/api_types";

  import type State from "src/modals/State";
  import type { WSStore } from "src/stores/ws";
  import { getContext } from "svelte";
  export let server: ServerInfo | null = null;

  let editing = false;
  let name = "Select a server.";

  $: if (!editing) {
    name = server?.name ?? "Select a server.";
  }

  const ws: WSStore<State> = getContext("ws");

  const change_name = (name: string, el: HTMLInputElement) => {
    editing = false;
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
    server.name = name;
    el.innerText = name;
  };
  const on_name_changed = (e: Event) => {
    const target = e.target as HTMLInputElement;
    change_name(target.innerText, target);
  }
  const on_name_key = (e: KeyboardEvent) => {
    if (e.key !== "Enter" || !e.currentTarget) return;
    e.preventDefault();
    const target = e.currentTarget as HTMLInputElement;
    target.blur();
    change_name(target.innerText, target);
  }
</script>

<article class:noserver="{server === null}">
  <header>
    {#if server}
      <h1
        on:focus="{() => editing = true}"
        on:blur="{on_name_changed}"
        on:keypress="{on_name_key}"
        disabled="{!server}"
        contenteditable="true"
        bind:textContent="{name}">
      </h1>  
    {:else}
      <h1>Select a server.</h1>
    {/if}
    <button>Edit</button>
    <button>Power</button>
    <button>Open</button>
    <span class="grow"/>
    <button>Delete</button>
  </header>
  <section>
    <ul>
      <li>
        <label for="comm_type">Communicator Type</label>
        <select name="comm_type" on:change="{()=>{}}">
          {#each Object.keys(CommunicatorType) as c}
            <option value="{c}">{c}</option>
          {/each}
        </select>
      </li>
      <li>
        <label for="comm_port">Port</label>
        <input type="text" name="comm_port" on:change="{()=>{}}"/>
      </li>
      <li>
        <label for="comm_ip">IP</label>
        <input type="text" name="comm_ip" on:change="{()=>{}}"/>
      </li>
    </ul>
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
    &:enabled:hover, &[contenteditable]:hover {
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
    align-items: flex-end;
    &>h1 {
      @include minimal-input;
      margin: 0 .5em 0 0;
      padding: 0.2em 0.3em;

      min-width: 2em;
      max-width: 10em;
      width: max-content;

      // word-break: break-all;
      word-wrap: break-word;
      overflow: hidden;

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
  }

  .noserver {
    button, section {
      visibility: hidden;
    }
  }

  section {
    margin: 2em 2em 1em 2em;

    ul {
      display: grid;
      grid-template-columns: 1fr 1fr;
      align-items: center;

      gap: 0.25em 0.5em;
      
      margin: 0;
      padding: 0;

      list-style: none;
      color: #888A8B;
      li {
        display: contents;
      }

      label {
        text-align: right;
      }

      select, input {
        @include minimal-input;
        height: 1.25em;
        font-size: 1rem;
      }

      select {
        padding-left: 0;
      }
    }
  }
</style>