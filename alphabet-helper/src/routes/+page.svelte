<script>
  import { Body } from 'svelte-body';
  import PaletteButton from '../PaletteButton.svelte';
  import GridCell from '../GridCell.svelte';
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

<div class="letter-draw-grid" style={paletteStyle}>
  <GridCell palette={paletteStyle} />
  <GridCell palette={paletteStyle} />
  <GridCell palette={paletteStyle} />
  <GridCell palette={paletteStyle} />
  <div></div>
  <GridCell palette={paletteStyle} />
  <GridCell palette={paletteStyle} />
  <GridCell palette={paletteStyle} />
  <GridCell palette={paletteStyle} />
</div>

<div class="alphabet-grid">
  <Letter palette={paletteStyle} letter="A" pattern="65" />
  <Letter palette={paletteStyle} letter="B" pattern="65" />
  <Letter palette={paletteStyle} letter="C" pattern="65" />
  <Letter palette={paletteStyle} letter="D" pattern="65" />
  <Letter palette={paletteStyle} letter="E" pattern="65" />
  <Letter palette={paletteStyle} letter="F" pattern="65" />
  <Letter palette={paletteStyle} letter="G" pattern="65" />
  <Letter palette={paletteStyle} letter="H" pattern="65" />
  <Letter palette={paletteStyle} letter="I" pattern="65" />
  <Letter palette={paletteStyle} letter="J" pattern="65" />
  <Letter palette={paletteStyle} letter="K" pattern="65" />
  <Letter palette={paletteStyle} letter="L" pattern="65" />
  <Letter palette={paletteStyle} letter="M" pattern="65" />
  <Letter palette={paletteStyle} letter="N" pattern="65" />
  <Letter palette={paletteStyle} letter="O" pattern="65" />
  <Letter palette={paletteStyle} letter="P" pattern="65" />
  <Letter palette={paletteStyle} letter="Q" pattern="65" />
  <Letter palette={paletteStyle} letter="R" pattern="65" />
  <Letter palette={paletteStyle} letter="S" pattern="65" />
  <Letter palette={paletteStyle} letter="T" pattern="65" />
  <Letter palette={paletteStyle} letter="U" pattern="65" />
  <Letter palette={paletteStyle} letter="V" pattern="65" />
  <Letter palette={paletteStyle} letter="W" pattern="65" />
  <Letter palette={paletteStyle} letter="X" pattern="65" />
  <Letter palette={paletteStyle} letter="Y" pattern="65" />
  <Letter palette={paletteStyle} letter="Z" pattern="65" />
</div>

<style>
  .palette-row {
    display: grid;
    grid-auto-flow: column;
  }

  .letter-draw-grid {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    width: max-content;
    border-style: solid;

    grid-gap: 50px;
    border-width: 50px;

    background-color: var(--color-light);
    border-top-color: var(--color-dark);
    border-left-color: var(--color-dark);
    border-bottom-color: var(--color-white);
    border-right-color: var(--color-white);
    color: var(--color-black);
  }

  .alphabet-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    place-items: center;
  }
</style>
