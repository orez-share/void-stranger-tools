<script>
  import PaletteRow from '$lib/../PaletteRow.svelte';
  import VoidLetter from '$lib/../VoidLetter.svelte';
  import Letter from '$lib/../Letter.svelte';

  let patternRequirement = {ones: 0, zeroes: 0};
  let paletteStyle;
</script>

<PaletteRow bind:paletteStyle />

<div class="draw-container">
  <VoidLetter bind:patternRequirement palette={paletteStyle} size="50px" />
</div>

<div class="alphabet-grid">
  <Letter letter="A" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="B" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="C" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="D" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="E" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="F" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="G" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="H" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="I" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="J" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="K" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="L" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="M" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="N" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="O" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="P" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="Q" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="R" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="S" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="T" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="U" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="V" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="W" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="X" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="Y" patternRequirement={patternRequirement} palette={paletteStyle} />
  <Letter letter="Z" patternRequirement={patternRequirement} palette={paletteStyle} />
</div>

<style>
  .palette-row {
    display: grid;
    grid-auto-flow: column;
    justify-items: center;
  }

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
