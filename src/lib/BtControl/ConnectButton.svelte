<script lang="ts">
  import { getToastStore, type ToastSettings } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api";
  const toastStore = getToastStore();

  export let index: number;

  let isConnecting = false;

  async function connect(index: number) {
    try {
      isConnecting = true;
      console.log("connecting");
      await invoke("connect", { index });
      let code = prompt("Input code");
      await invoke("authenticate", { code });
      isConnecting = false;
    } catch (err: any) {
      isConnecting = false;
      const t: ToastSettings = {
        message: err,
        background: "variant-filled-error",
      };
      toastStore.trigger(t);
    }
  }
</script>

{#if !isConnecting}
  <button
    type="button"
    class="btn variant-filled-primary"
    on:click={() => connect(index)}
  >
    Connect
  </button>
{:else}
  <button type="button" class="btn variant-filled-surface"> Connect </button>
{/if}

<!--{#if isConnecting}{:else if isAuthenticating}{:else}{/if}-->

<!--<div />-->
