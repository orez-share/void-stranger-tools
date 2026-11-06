<script>
  import { base } from '$app/paths';
  import PaletteRow from '$lib/../PaletteRow.svelte';
  let paletteStyle;
</script>

<PaletteRow bind:paletteStyle />

<div style={paletteStyle}>
  <h1>Void Stranger Tools</h1>

  <ul>
  <li><a href="{base}/alphabet" style={paletteStyle}>Alphabet Helper</a></li>
  <li><a href="{base}/mural" style={paletteStyle}>Mural Maker</a></li>
  </ul>
</div>

<style>
  div {
    font-family: Arial;
    color: var(--color-black);
  }

  ul {
    line-height: 3em;
  }

  a {
    font-size: 1.5em;
    color: var(--color-black);
  }
</style>
