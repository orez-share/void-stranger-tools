<script>
  import VoidLetter from './VoidLetter.svelte';
  export let palette;
  export let letter;
  export let pattern;
</script>

<div class="container">
  <VoidLetter disabled palette={palette} pattern={pattern} />
  { letter }
</div>

<style>
  .container {
    margin-bottom: 30px;
  }
</style>
