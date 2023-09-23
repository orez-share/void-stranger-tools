<script>
  import VoidLetter from './VoidLetter.svelte';
  export let palette;
  export let letter;
  export let pattern;
  export let patternRequirement;

  pattern = +pattern;

  let faded;
  $: faded = (
    ((patternRequirement.ones & pattern) != patternRequirement.ones) ||
    ((patternRequirement.zeroes & pattern) != 0)
  );
</script>

<div style={palette} class="container">
  <VoidLetter faded={faded} disabled palette={palette} pattern={pattern} />
  { letter }
</div>

<style>
  .container {
    margin-bottom: 30px;
    color: var(--color-black);
  }
</style>
