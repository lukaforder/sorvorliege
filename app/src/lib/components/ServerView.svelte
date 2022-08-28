<script lang="ts">

</script>

<article>
  <header>
    <h1>CSGO Server</h1>
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
    margin: 2em 1em 1em 1em;
    align-items: baseline;
    h1 {
      margin: 0;
      margin-right: 0.5em;
    }
    * {
      padding: 0.5em 0.8em;
    }
    border-bottom: 1px solid #2E3338;
  }
</style>