<script>
  import { Body } from 'svelte-body';
  import PaletteButton from '../PaletteButton.svelte';
  import VoidLetter from '../VoidLetter.svelte';
  import Letter from '../Letter.svelte';

  const paletteToStyle = palette => `
    --color-black: ${palette.black};
    --color-dark: ${palette.dark};
    --color-light: ${palette.light};
    --color-white: ${palette.white};
  `;
  const setPalette = pal => palette = pal;
  const gray = {
    black: "#000",
    dark: "#808080",
    light: "#C0C0C0",
    white: "#FFF",
  };

  let palette = gray;
  $: bgColor = `background-color: ${palette.light}`;
  $: paletteStyle = paletteToStyle(palette);
</script>

<svelte:head>
  <title>Void Stranger Alphabet Tool</title>
</svelte:head>
<Body style={bgColor} />
<div class="palette-row">
  <PaletteButton handleSelect={setPalette} name="Gray" palette={gray} />
  <PaletteButton handleSelect={setPalette} name="R***" palette={{dark: "#c10d0c", light: "#f77b57"}} />
  <PaletteButton handleSelect={setPalette} name="O***" palette={{dark: "#bf5a3c", light: "#f9a140"}} />
  <PaletteButton handleSelect={setPalette} name="Y***" palette={{dark: "#ba779a", light: "#e9c300"}} />
  <PaletteButton handleSelect={setPalette} name="G***" palette={{black: "#001413", dark: "#097264", light: "#58dd8a"}} />
  <PaletteButton handleSelect={setPalette} name="B***" palette={{black: "#162b30", dark: "#087594", light: "#64c8fa"}} />
  <PaletteButton handleSelect={setPalette} name="I***" palette={{black: "#0e0021", dark: "#754672", light: "#a799cc"}} />
  <PaletteButton handleSelect={setPalette} name="V***" palette={{black: "#05001e", dark: "#6630a0", light: "#ba81c8"}} />
</div>

<VoidLetter palette={paletteStyle} size="50px" />

<div class="alphabet-grid">
  <Letter palette={paletteStyle} letter="A" pattern="65" />
  <Letter palette={paletteStyle} letter="B" pattern="129" />
  <Letter palette={paletteStyle} letter="C" pattern="193" />
  <Letter palette={paletteStyle} letter="D" pattern="33" />
  <Letter palette={paletteStyle} letter="E" pattern="97" />
  <Letter palette={paletteStyle} letter="F" pattern="161" />
  <Letter palette={paletteStyle} letter="G" pattern="225" />
  <Letter palette={paletteStyle} letter="H" pattern="17" />
  <Letter palette={paletteStyle} letter="I" pattern="81" />
  <Letter palette={paletteStyle} letter="J" pattern="145" />
  <Letter palette={paletteStyle} letter="K" pattern="209" />
  <Letter palette={paletteStyle} letter="L" pattern="49" />
  <Letter palette={paletteStyle} letter="M" pattern="114" />
  <Letter palette={paletteStyle} letter="N" pattern="177" />
  <Letter palette={paletteStyle} letter="O" pattern="241" />
  <Letter palette={paletteStyle} letter="P" pattern="9" />
  <Letter palette={paletteStyle} letter="Q" pattern="73" />
  <Letter palette={paletteStyle} letter="R" pattern="137" />
  <Letter palette={paletteStyle} letter="S" pattern="201" />
  <Letter palette={paletteStyle} letter="T" pattern="41" />
  <Letter palette={paletteStyle} letter="U" pattern="105" />
  <Letter palette={paletteStyle} letter="V" pattern="169" />
  <Letter palette={paletteStyle} letter="W" pattern="233" />
  <Letter palette={paletteStyle} letter="X" pattern="25" />
  <Letter palette={paletteStyle} letter="Y" pattern="89" />
  <Letter palette={paletteStyle} letter="Z" pattern="153" />
</div>

<style>
  .palette-row {
    display: grid;
    grid-auto-flow: column;
  }

  .alphabet-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    place-items: center;
  }
</style>
