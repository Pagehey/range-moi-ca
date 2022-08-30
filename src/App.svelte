<script lang="ts">
  import Confirm from './lib/components/Confirm.svelte';
  import Error from './lib/components/Error.svelte';
  import Success from './lib/components/Success.svelte';
  import Select from './lib/components/Select.svelte';

  import { open } from '@tauri-apps/api/dialog';
  import { invoke } from '@tauri-apps/api/tauri';
  import type { CommandResponse } from './lib/interfaces/CommandResponse';
  import { CommandSuccess } from './lib/interfaces/CommandResponse';

  function selectFolder() {
    open({ directory: true }).then((selected) => {
      if (selected) {
        invoke('select_folder', { folderPath: selected })
          .then((response: CommandResponse) => {
            if (CommandSuccess(response.status)) {
              folderPath = <string> selected;
              message = response.message;
            } else {
              error = true;
              message = response.message;
            }
          });
      }
    });
  }

  let folderPath: string;
  let message: string;
  let error: boolean;
  let done: boolean;

  const reset = () => folderPath = message = error = done = null;

  function restart() {
    reset();
    selectFolder();
  }

  function throwError(errorMessage: string = '') {
    error = true;
    message = errorMessage;
  }
</script>

<main>
  {#if error}
    <Error {message} on:retry={reset} />
  {:else if done}
    <Success {folderPath} on:restart={restart} />
  {:else if folderPath}
    <Confirm
      {folderPath}
      {message}
      on:cancel={reset}
      on:success|once={() => done = true}
      on:error|once={(e) => throwError(e.detail.message)}
    />
  {:else}
    <Select on:selectFolder={selectFolder} />
  {/if}
</main>

<style>
  main {
    height: 100%;
  }
</style>
