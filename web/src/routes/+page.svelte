<script>
  import PaletteRow from '$lib/../PaletteRow.svelte';
  let paletteStyle;
</script>

<PaletteRow bind:paletteStyle />

<h1>Void Stranger Tools</h1>

<ul>
<li><a href="alphabet" style={paletteStyle}>Alphabet Helper</a></li>
<li><a href="mural" style={paletteStyle}>Mural Maker</a></li>
</ul>

<style>
  ul {
    line-height: 3em;
  }

  a {
    font-size: 1.5em;
    color: var(--color-black);
  }
</style>
