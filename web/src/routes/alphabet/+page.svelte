<script>
  import PaletteRow from '$lib/../PaletteRow.svelte';
  import VoidLetter from './VoidLetter.svelte';
  import Letter from './Letter.svelte';

  let patternRequirement = {ones: 0, zeroes: 0};
  let paletteStyle;
</script>

<PaletteRow bind:paletteStyle />

<div class="draw-container">
  <VoidLetter bind:patternRequirement palette={paletteStyle} size="50px" />
</div>

<div class="alphabet-grid">
  <Letter letter="A" {patternRequirement} palette={paletteStyle} />
  <Letter letter="B" {patternRequirement} palette={paletteStyle} />
  <Letter letter="C" {patternRequirement} palette={paletteStyle} />
  <Letter letter="D" {patternRequirement} palette={paletteStyle} />
  <Letter letter="E" {patternRequirement} palette={paletteStyle} />
  <Letter letter="F" {patternRequirement} palette={paletteStyle} />
  <Letter letter="G" {patternRequirement} palette={paletteStyle} />
  <Letter letter="H" {patternRequirement} palette={paletteStyle} />
  <Letter letter="I" {patternRequirement} palette={paletteStyle} />
  <Letter letter="J" {patternRequirement} palette={paletteStyle} />
  <Letter letter="K" {patternRequirement} palette={paletteStyle} />
  <Letter letter="L" {patternRequirement} palette={paletteStyle} />
  <Letter letter="M" {patternRequirement} palette={paletteStyle} />
  <Letter letter="N" {patternRequirement} palette={paletteStyle} />
  <Letter letter="O" {patternRequirement} palette={paletteStyle} />
  <Letter letter="P" {patternRequirement} palette={paletteStyle} />
  <Letter letter="Q" {patternRequirement} palette={paletteStyle} />
  <Letter letter="R" {patternRequirement} palette={paletteStyle} />
  <Letter letter="S" {patternRequirement} palette={paletteStyle} />
  <Letter letter="T" {patternRequirement} palette={paletteStyle} />
  <Letter letter="U" {patternRequirement} palette={paletteStyle} />
  <Letter letter="V" {patternRequirement} palette={paletteStyle} />
  <Letter letter="W" {patternRequirement} palette={paletteStyle} />
  <Letter letter="X" {patternRequirement} palette={paletteStyle} />
  <Letter letter="Y" {patternRequirement} palette={paletteStyle} />
  <Letter letter="Z" {patternRequirement} palette={paletteStyle} />
</div>

<style>
  .draw-container {
    text-align: center;
    margin-top: 30px;
    margin-bottom: 30px;
  }

  .alphabet-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    place-items: center;
  }
</style>
