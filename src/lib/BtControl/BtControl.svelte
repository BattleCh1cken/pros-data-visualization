<script lang="ts">
  import {
    ListBox,
    ListBoxItem,
    Drawer,
    drawerStore,
  } from "@skeletonlabs/skeleton";
  import type { DrawerSettings } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api";

  import ArrowLeft from "svelte-material-icons/ArrowLeft.svelte";

  let closeDrawer = () => {
    drawerStore.close();
  };

  let handleSearch = async () => {
    console.log("looking for brains");
    let bob: String;
    try {
      bob = await invoke("setup");
    } catch (error) {
      console.log(error);
    }
  };

  interface Brain {
    name: String;
    uuid: String;
  }

  let brains: Brain[] = [
    {
      name: "Ares",
      uuid: "08590f7e-db05-467e-8757-72f6faeb1306",
    },
    {
      name: "Bob",
      uuid: "08590f7e-db05-467e-8757-72f6faeb1306",
    },
  ];
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
    <button
      type="button"
      class="btn variant-filled-primary"
      on:click={handleSearch}>Search</button
    >
    <div class="card p-4 flex flex-col">
      {#each brains as brain, index}
        <div class="card p-2">
          <p>Name: {brain.name}</p>
          <p>UUID: {brain.uuid}</p>
          <button type="button" class="btn variant-filled-primary"
            >Connect</button
          >
        </div>
      {/each}
    </div>
    <button type="button" class="btn variant-filled-primary">Disconnect</button>
  </div>
</Drawer>
