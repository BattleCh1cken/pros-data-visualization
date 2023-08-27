<script lang="ts">
  import "./app.postcss";

  import { Toast } from "@skeletonlabs/skeleton";

  import { AppShell, initializeStores } from "@skeletonlabs/skeleton";
  initializeStores();
  import { afterUpdate } from "svelte";
  import { listen } from "@tauri-apps/api/event";

  import Bar from "./lib/Bar.svelte";
  import BtControl from "./lib/BtControl/Drawer.svelte";
  //import Graph from "./lib/PidGraph.svelte";
  import Graph from "./lib/Graph.svelte";
  import TestGraph from "./lib/TestGraph.svelte";

  let data: number[][] = [[1], [1], [1]];

  afterUpdate(() => {
    listen<number[][]>("chart_data_update", (event) => {
      data = event.payload;
    });
  });
</script>

<Toast />

<BtControl />

<AppShell>
  <svelte:fragment slot="header">
    <Bar />
  </svelte:fragment>
  <Graph rows={["Left", "Right"]} chartData={data} />
</AppShell>
