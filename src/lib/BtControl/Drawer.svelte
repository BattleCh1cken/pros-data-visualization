<script lang="ts">
  import {
    ListBox,
    ListBoxItem,
    Drawer,
    drawerStore,
  } from "@skeletonlabs/skeleton";
  import type { DrawerSettings } from "@skeletonlabs/skeleton";
  import { afterUpdate, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { listen } from "@tauri-apps/api/event";

  import ArrowLeft from "svelte-material-icons/ArrowLeft.svelte";

  import SearchButton from "./SearchButton.svelte";
  import ConnectButton from "./ConnectButton.svelte";

  let closeDrawer = () => {
    drawerStore.close();
  };

  interface Brain {
    name: String;
  }

  let brains: Brain[] = [];

  afterUpdate(() => {
    listen<Brain[]>("found_brain", (event) => {
      console.log(event.payload);
      brains = event.payload;
    });
  });
</script>

<Drawer position="left" rounded="rounded-xl" padding="p-4" width="w-[540px]">
  <button
    type="button"
    class="btn-icon variant-transparent"
    on:click={closeDrawer}
  >
    <ArrowLeft size={30} />
  </button>

  <h3 class="h3 text-center">Bluetooth Control</h3>

  <div class="flex flex-col align-center justify-center m-4 gap-4">
    <div class="card p-4 flex flex-col">
      {#if brains.length === 0}
        No brains found.
      {:else}
        {#each brains as brain, index}
          <div class="card p-2">
            <p>Name: {brain.name}</p>
            <ConnectButton {index} />
          </div>
        {/each}
      {/if}
    </div>
    <button type="button" class="btn variant-filled-primary">Disconnect</button>
  </div>
</Drawer>
