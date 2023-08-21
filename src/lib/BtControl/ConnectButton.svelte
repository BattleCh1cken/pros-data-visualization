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

<!--click connect button-->
<!--check if brain is accepting connection request-->
<!--display code on brain-->
<!--display input box-->
<!--check if input is valid-->

<!--{#if is_code_valid(input_code) == InputStatus.Invalid}-->
<!--<input-->
<!--class="input input-error"-->
<!--title="Code"-->
<!--type="text"-->
<!--placeholder="input code"-->
<!--bind:value={input_code}-->
<!--/>-->
<!--{:else if is_code_valid(input_code) == InputStatus.Valid}-->
<!--<input-->
<!--class="input input-success"-->
<!--title="Code"-->
<!--type="text"-->
<!--placeholder="input code"-->
<!--bind:value={input_code}-->
<!--/>-->
<!--{:else}-->
<!--<input-->
<!--class="input"-->
<!--title="Code"-->
<!--type="text"-->
<!--placeholder="input code"-->
<!--bind:value={input_code}-->
<!--/>-->
<!--{/if}-->

<button
  type="button"
  class="btn variant-filled-primary"
  on:click={() => connect(index)}>Connect</button
>

<!--{#if isConnecting}{:else if isAuthenticating}{:else}{/if}-->

<!--<div />-->
