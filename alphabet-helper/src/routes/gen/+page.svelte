<script>
  import { Body } from 'svelte-body';
  import Mural from '../../Mural.svelte';
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

<svelte:head>
  <title>Void Stranger Alphabet Tool</title>
  <!-- <link rel="icon" href={pagePalette.icon} /> -->
</svelte:head>
<!-- <Body style={bgColor} /> -->
<Mural {text} {brandPattern} />
<input type="text" bind:value={text} />
<button on:click={randomizePattern}>Randomize Pattern</button>

<style>
  input {
    text-transform: uppercase;
    width: 300px;
  }
</style>
