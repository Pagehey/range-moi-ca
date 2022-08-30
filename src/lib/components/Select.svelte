<script lang="ts">
import { createEventDispatcher } from "svelte";


  const dispatch = createEventDispatcher();
</script>

<div class="select-folder">
  <button on:click={() => dispatch('selectFolder')}>
    Choisir un dossier
  </button>
</div>

<style scoped>
  .select-folder {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
