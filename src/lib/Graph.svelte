<script lang="ts">
  import { Chart, registerables } from "chart.js";
  import { afterUpdate } from "svelte";

  Chart.register(...registerables);

  export let rows: string[] = [""];
  export let chartData: number[][] = [[0], [0]];

  let chart: Chart;
  let chartElement: HTMLCanvasElement;

  afterUpdate(() => {
    let labels = chartData[0];
    let datasets = rows.map((name, index) => {
      return {
        label: name,
        data: chartData[index + 1],
        pointRadius: 0,
        spanGaps: true,
      };
    });

    let data = {
      labels,
      datasets,
    };

    if (chart) chart.destroy();
    chart = new Chart(chartElement, {
      type: "line",
      data,
      options: {
        animation: false,
        plugins: {
          decimation: {
            enabled: true,
          },
        },
      },
    });
  });
</script>

<div class="card p-2">
  <canvas bind:this={chartElement} />
</div>
