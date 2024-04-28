<script>
  import { Body } from 'svelte-body';
  import PaletteRow from '$lib/../PaletteRow.svelte';
  import Mural from '$lib/../Mural.svelte';
  let brandPattern;
  let text = "BE SURE TO DRINK YOUR OVALTINE";

  // Durstenfeld shuffle
  // https://stackoverflow.com/a/12646864/1163020
  const randomizePattern = () => {
    let len = text.replaceAll(" ", "").length;
    let pattern = (1n << BigInt(len)) - 1n;
    let index1 = 36;
    while (index1--) {
      const index2 = Math.floor((index1 + 1) * Math.random());
      const bit1 = 1n << BigInt(index1);
      const bit2 = 1n << BigInt(index2);
      if (!(pattern & bit1) != !(pattern & bit2)) {
        pattern ^= bit1 | bit2;
      }
    }
    brandPattern = pattern;
  }
  randomizePattern();
</script>

<PaletteRow />
<Mural {text} {brandPattern} />
<input type="text" bind:value={text} />
<button on:click={randomizePattern}>Randomize Pattern</button>

<style>
  input {
    text-transform: uppercase;
    width: 300px;
  }
</style>
