<script>
  import VoidLetter from './VoidLetter.svelte';
  export let palette;
  export let letter;
  export let pattern;
</script>

<div style={palette} class="container">
  <VoidLetter disabled palette={palette} pattern={pattern} />
  { letter }
</div>

<style>
  .container {
    margin-bottom: 30px;
    color: var(--color-black);
  }
</style>
