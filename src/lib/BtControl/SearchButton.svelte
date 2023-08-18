<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import {
    Toast,
    toastStore,
    type ToastSettings,
    ProgressRadial,
    ProgressBar,
  } from "@skeletonlabs/skeleton";

  let isFetching = false;

  const fetchData = async () => {
    try {
      isFetching = true;
      await invoke("setup");
    } catch (error) {
      isFetching = false;
      const toastSettings: ToastSettings = {
        message: error,
        background: "variant-filled-warning",
      };
      toastStore.trigger(toastSettings);
    } finally {
      isFetching = false;
    }
  };
</script>

{#if isFetching}
  <button type="button" class="btn variant-soft-primary h-10">
    <ProgressBar />
  </button>
{:else}
  <button type="button" class="btn variant-filled-primary h-10" on:click={fetchData}>
    Search
  </button>
{/if}
