<script lang="ts">
  import { Chart } from "chart.js/auto";
  import { onMount } from "svelte";

  function createChart() {
    let ctx = document.getElementById("myChart");
    let chart = new Chart(ctx, {
      type: "line",
      data: {
        labels: ["Red", "Blue", "Yellow", "Green", "Purple", "Orange"],
        datasets: [
          {
            label: "# of Votes",
            data: [12, 19, 3, 5, 2, 3],
            borderWidth: 1,
          },
        ],
      },
      options: {
        scales: {
          y: {
            beginAtZero: true,
          },
        },
      },
    });
  }

  onMount(createChart);
</script>

<div>
  <canvas id="myChart" />
</div>
