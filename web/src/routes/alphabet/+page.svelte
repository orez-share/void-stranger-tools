<script>
  import grayIcon from '$lib/assets/gray.gif';
  import rIcon from '$lib/assets/red.gif';
  import oIcon from '$lib/assets/orange.gif';
  import yIcon from '$lib/assets/yellow.gif';
  import gIcon from '$lib/assets/green.gif';
  import bIcon from '$lib/assets/blue.gif';
  import iIcon from '$lib/assets/indigo.gif';
  import vIcon from '$lib/assets/violet.gif';

  import { Body } from 'svelte-body';
  import PaletteButton from '$lib/../PaletteButton.svelte';
  import VoidLetter from '$lib/../VoidLetter.svelte';
  import Letter from '$lib/../Letter.svelte';

  const gray = {
    black: "#000",
    dark: "#808080",
    light: "#C0C0C0",
    white: "#FFF",
    icon: grayIcon,
  };

  let patternRequirement = {ones: 0, zeroes: 0};
  let bgColor;
  let paletteStyle;
  let pagePalette = gray;
  $: bgColor = `background-color: ${pagePalette.light}`;
  $: paletteStyle = `
    --color-black: ${pagePalette.black};
    --color-dark: ${pagePalette.dark};
    --color-light: ${pagePalette.light};
    --color-white: ${pagePalette.white};
  `;
</script>

<svelte:head>
  <title>Void Stranger Alphabet Tool</title>
  <link rel="icon" href={pagePalette.icon} />
</svelte:head>
<Body style={bgColor} />
<div class="palette-row">
  <PaletteButton bind:pagePalette name="Gray" palette={gray} />
  <PaletteButton bind:pagePalette name="R***" palette={{dark: "#c10d0c", light: "#f77b57", icon: rIcon}} />
  <PaletteButton bind:pagePalette name="O***" palette={{dark: "#bf5a3c", light: "#f9a140", icon: oIcon}} />
  <PaletteButton bind:pagePalette name="Y***" palette={{dark: "#ba779a", light: "#e9c300", icon: yIcon}} />
  <PaletteButton bind:pagePalette name="G***" palette={{black: "#001413", dark: "#097264", light: "#58dd8a", icon: gIcon}} />
  <PaletteButton bind:pagePalette name="B***" palette={{black: "#162b30", dark: "#087594", light: "#64c8fa", icon: bIcon}} />
  <PaletteButton bind:pagePalette name="I***" palette={{black: "#0e0021", dark: "#754672", light: "#a799cc", icon: iIcon}} />
  <PaletteButton bind:pagePalette name="V***" palette={{black: "#05001e", dark: "#6630a0", light: "#ba81c8", icon: vIcon}} />
</div>

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
