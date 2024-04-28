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
  import { get } from 'svelte/store';
  import { persisted } from 'svelte-persisted-store';
  import PaletteButton from './PaletteButton.svelte';

  const gray = {
    black: "#000",
    dark: "#808080",
    light: "#C0C0C0",
    white: "#FFF",
    icon: grayIcon,
  };

  export const pagePalette = persisted('pagePalette', gray);
  export let paletteStyle;
  let bgColor;
  $: bgColor = `background-color: ${$pagePalette.light}`;
  $: paletteStyle = `
    --color-black: ${$pagePalette.black};
    --color-dark: ${$pagePalette.dark};
    --color-light: ${$pagePalette.light};
    --color-white: ${$pagePalette.white};
  `;
</script>

<svelte:head>
  <title>Void Stranger Alphabet Tool</title>
  <link rel="icon" href={$pagePalette.icon} />
</svelte:head>
<Body style={bgColor} />

<div class="palette-row-wrapper">
  <div class="palette-row">
    <div class="home"><a href="/"><img alt="Home" src={$pagePalette.icon} /></a></div>
    <PaletteButton {pagePalette} name="Gray" palette={gray} />
    <PaletteButton {pagePalette} name="R***" palette={{dark: "#c10d0c", light: "#f77b57", icon: rIcon}} />
    <PaletteButton {pagePalette} name="O***" palette={{dark: "#bf5a3c", light: "#f9a140", icon: oIcon}} />
    <PaletteButton {pagePalette} name="Y***" palette={{dark: "#ba779a", light: "#e9c300", icon: yIcon}} />
    <PaletteButton {pagePalette} name="G***" palette={{black: "#001413", dark: "#097264", light: "#58dd8a", icon: gIcon}} />
    <PaletteButton {pagePalette} name="B***" palette={{black: "#162b30", dark: "#087594", light: "#64c8fa", icon: bIcon}} />
    <PaletteButton {pagePalette} name="I***" palette={{black: "#0e0021", dark: "#754672", light: "#a799cc", icon: iIcon}} />
    <PaletteButton {pagePalette} name="V***" palette={{black: "#05001e", dark: "#6630a0", light: "#ba81c8", icon: vIcon}} />
  </div>
</div>

<style>
  .home img {
    width: 56px;
    image-rendering: pixelated;
  }

  .home a {
    height: 0;
    display: inherit;
  }

  .palette-row {
    display: grid;
    grid-auto-flow: column;
    justify-items: center;
  }

  .palette-row-wrapper {
    margin-bottom: 30px;
  }
</style>
