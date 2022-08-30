<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let message: string;

  const dispatch = createEventDispatcher();
</script>

<section>
  <div class="error">
    <p><strong>Erreur !</strong></p>
    <p>{message}</p>
  </div>
  <div>
    <button
      type="button"
      on:click={() => dispatch('retry')}
    >Réessayer</button>
  </div>
</section>

<style scoped>
  section {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
  }
  .error {
    width: 80%;
    min-width: 280px;
    padding: .5rem 1rem;
    background-color: #FECACA;
    color: #B91C1C;
    border-radius: 8px;
  }

  .error p {
    white-space: pre-wrap;
  }

  button {
    margin-top: 1rem;
  }
</style>
