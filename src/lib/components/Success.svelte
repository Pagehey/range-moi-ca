<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { open } from '@tauri-apps/api/shell';

  export let folderPath: string;

  function openFolder() {
    console.log(folderPath);
    open(folderPath);
  }

  const dispatch = createEventDispatcher();
</script>

<section>
  <p>Terminé !</p>
  <button
    type="button"
    on:click={() => dispatch('restart')}
  >Trier un autre dossier</button>
  <a href="/" on:click|preventDefault={openFolder}>Ouvrir le dossier</a>
</section>

<style scoped>
  section {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
  }

  p {
    padding: .5rem 1rem;
    background-color: #F0FDF4;
    color: #166534;
    border-radius: 8px;
    text-align: center;
    font-weight: bold;
  }

  a {
    margin-top: .5rem;
    color: black;
  }

  a:hover {
      color: rgb(60, 60, 60);
    }

  @media (prefers-color-scheme: dark) {
    a {
      color: white;
    }

    a:hover {
      color: rgb(230, 230, 230);
    }
  }
</style>
