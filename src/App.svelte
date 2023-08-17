<script lang="ts">
  import "@skeletonlabs/skeleton/themes/theme-crimson.css";
  import "@skeletonlabs/skeleton/styles/skeleton.css";

  import { AppShell, AppBar } from "@skeletonlabs/skeleton";

  import "./app.css";

  import Bar from "./lib/Bar.svelte";
  import BtControl from "./lib/BtControl.svelte";
  import Graph from "./lib/Graph.svelte";
</script>

<BtControl />

<AppShell>
  <svelte:fragment slot="header">
    <Bar />
  </svelte:fragment>
  <Graph />
</AppShell>
