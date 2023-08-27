<script lang="ts">
  import { AppBar } from "@skeletonlabs/skeleton";
  import { getDrawerStore } from "@skeletonlabs/skeleton";
  import Menu from "svelte-material-icons/Menu.svelte";

  const drawerStore = getDrawerStore();

  let openDrawer = () => {
    drawerStore.open();
  };
</script>

<AppBar
  gridColumns="grid-cols-3"
  slotDefault="place-self-center"
  slotTrail="place-content-end"
>
  <svelte:fragment slot="lead">
    <button type="button" class="btn-icon variant-ghost" on:click={openDrawer}>
      <Menu size={30} />
    </button>
  </svelte:fragment>
  <h1 class="h2">Viewinator</h1>
</AppBar>
