<script>
  import VoidLetter from './VoidLetter.svelte';
  export let palette;
  export let letter;
  export let patternRequirement;

  let pattern = letter.charCodeAt(0);

  let faded;
  $: faded = (
    ((patternRequirement.ones & pattern) != patternRequirement.ones) ||
    ((patternRequirement.zeroes & pattern) != 0)
  );
</script>

<div style={palette} class="container">
  <VoidLetter {faded} disabled {palette} {pattern} />
  { letter }
</div>

<style>
  .container {
    margin-bottom: 30px;
    color: var(--color-black);
  }
</style>
