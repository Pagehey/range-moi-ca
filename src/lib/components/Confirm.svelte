<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import type { CommandResponse } from '../interfaces/CommandResponse';
  import { CommandSuccess } from '../interfaces/CommandResponse';

  export let folderPath: string;
  export let message: string;

	const dispatch = createEventDispatcher();

  const cancel = () => dispatch('cancel');

  function confirm() {
    invoke('group_files', { folderPath })
      .then((response: CommandResponse) => {
        if (CommandSuccess(response.status)) {
            dispatch('success');
        } else {
          dispatch('error', { message: response.message });
        }
      })
      .catch((error) => {
        dispatch('error', { message: error });
      });
  }
</script>

<section>
  <div class="info">
    <pre class="path" title={folderPath}>{folderPath}</pre>
    <p class="message">{message}</p>
  </div>
  <div class="actions">
    <button
      type="button"
      on:click={cancel}
    >Annuler</button>
    <button
      type="button"
      class="primary ml"
      on:click={confirm}
    >Confirmer</button>
  </div>
</section>

<style scoped>
  section {
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 16px;
    justify-content: space-between;
  }

  .info {
    flex: auto 1 0;
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
  }

  .actions {
    flex: auto 0 1;
    display: flex;
    justify-content: flex-end;
  }

  .path {
    margin: 0;
    padding: 8px 12px 6px;
    background-color: rgb(198, 198, 198);
    border-radius: 8px;
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.2);
    font-family: monospace;
    line-height: 1rem;
    line-height: 1rem;
    text-overflow: ellipsis;
    overflow: hidden;
  }

  .ml {
    margin-left: .5rem;
  }

  .message {
    padding: 0 40px;
    color: black;
    white-space: pre-wrap;
  }

  @media (prefers-color-scheme: dark) {
    .message {
      color: white;
    }
  }
</style>
