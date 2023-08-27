<script lang="ts">
  import { Chart, registerables } from "chart.js";
  import { afterUpdate } from "svelte";

  Chart.register(...registerables);

  export let rows: string[] = [];
  export let chartData: number[][] = [];

  let chart: Chart;
  let chartElement: HTMLCanvasElement;

  afterUpdate(() => {
    let labels = chartData[0];
    let datasets = chartData.slice(1, chartData.length).map((array, index) => {
      return {
        label: rows[index],
        data: array,
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

<div>
  <canvas bind:this={chartElement} />
</div>
