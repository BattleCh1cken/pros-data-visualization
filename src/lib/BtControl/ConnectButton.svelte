<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  export let index: number;

  let isConnecting = false;
  let isAuthenticating = false;
  let isCodeValid = false;

  let input_code: number;

  enum InputStatus {
    Valid,
    Invalid,
    Empty,
  }

  function is_code_valid(code: number) {
    if (code == null) {
      return InputStatus.Empty;
    } else if (code.toString().length == 4) {
      return InputStatus.Valid;
    } else {
      return InputStatus.Invalid;
    }
  }

  async function connect(index: number) {
    console.log("connecting");
    await invoke("connect", { index });
    let code = prompt("Input code");
    await invoke("authenticate", { code });
  }
</script>

<button
  type="button"
  class="btn variant-filled-primary"
  on:click={() => connect(index)}>Connect</button
>

<!--{#if isConnecting}{:else if isAuthenticating}{:else}{/if}-->

<!--<div />-->
