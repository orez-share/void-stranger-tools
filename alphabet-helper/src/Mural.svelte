<script>
  import grayIcon from '$lib/assets/gray.gif';
  import { onMount } from 'svelte';
  import brandMural from '$lib/assets/brand-mural.gif';

  export let brandPattern;
  export let text;
  let canvas;
  let context;

  const scale = 4;

  // JS numbers have enough precision to keep a 36 bit number correctly,
  // but its bitwise operators truncate to a signed 32 bit number
  // because JS is a spiteful language.
  // We use BigInts instead, which is wasteful but ergonomic.
  // ᖍ(∙⟞∙)ᖌ
  // https://stackoverflow.com/q/33137519/1163020
  const MAX_IDX = 36n;
  const gray = {
    black: "#000",
    dark: "#808080",
    light: "#C0C0C0",
    white: "#FFF",
    icon: grayIcon,
  };

  const brandX = 88; // he's been using brand x!
  const brandY = 24;
  const brandSize = 48;

  const posToCoord = (position) => {
    const pos = Number(position);
    const cx = 5 - Math.floor(pos / 6);
    const cy = pos % 6;
    return {cx, cy}
  }

  const clear = () => {
    context.fillStyle = gray.light;
    context.fillRect(brandX, brandY, brandSize, brandSize);
  }

  const drawBit = (set, x, y) => {
    context.fillStyle = set ? gray.white : gray.dark;
    context.fillRect(x, y, 1, 1);
  }

  const drawChar = (chr, position) => {
    const pattern = chr ? chr.charCodeAt(0) : 0;
    const {cx, cy} = posToCoord(position);

    const x = brandX + cx * 8;
    const y = brandY + cy * 8;
    // clear area
    context.fillStyle = gray.light;
    context.fillRect(x, y, 7, 7);
    // surrounding bevel
    context.fillStyle = gray.white;
    context.fillRect(x + 6, y, 1, 7);
    context.fillRect(x, y + 6, 7, 1);
    context.fillStyle = gray.dark;
    context.fillRect(x, y, 7, 1);
    context.fillRect(x, y, 1, 7);
    // bits
    drawBit(pattern & 64, x + 1, y + 1);
    drawBit(pattern & 128, x + 3, y + 1);
    drawBit(pattern & 32, x + 5, y + 1);
    drawBit(pattern & 16, x + 1, y + 3);
    // 3 3 intentionally left blank
    drawBit(pattern & 8, x + 5, y + 3);
    drawBit(pattern & 4, x + 1, y + 5);
    drawBit(pattern & 1, x + 3, y + 5);
    drawBit(pattern & 2, x + 5, y + 5);
  }

  const redraw = (text, brandPattern) => {
    if (!context) return;
    clear();
    let idx = null;
    const nextIdx = () => {
      idx = idx != null ? idx + 1n : 0n;
      for (;; idx++) {
        let bit = 1n << idx;
        if (brandPattern & bit) return idx;
        if (idx >= MAX_IDX) return null;
      }
    };

    // draw text
    for (const chr of text.toUpperCase()) {
      if (chr == ' ') continue;
      const idx = nextIdx();
      if (idx == null) break;
      drawChar(chr, idx);
    }
    // draw remaining spaces
    while (true) {
      const idx = nextIdx();
      if (idx == null) break;
      drawChar(0, idx);
    }
  }

  $: redraw(text, brandPattern);

  onMount(() => {
    context = canvas.getContext('2d');
    const image = new Image();
    image.src = brandMural;

    image.onload = () => {
      context.drawImage(image, 0, 0);
      context.lineWidth = 1;
      redraw(text, brandPattern);
    }
  })
</script>

<div>
  <canvas
    width="224px"
    height="144px"
    bind:this={canvas}
    style="--scale: {scale};"
  />
</div>

<style>
  canvas {
    image-rendering: pixelated;
    width: calc(var(--scale) * 224px);
  }
</style>
