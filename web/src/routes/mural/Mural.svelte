<script>
  import { onMount } from 'svelte';
  import brandMural from '$lib/assets/brand-mural.gif';

  export let pagePalette;
  export let brandPattern;
  export let text;
  let canvas;
  let context;
  let grayImage;

  const scale = 4;

  // JS numbers have enough precision to keep a 36 bit number correctly,
  // but its bitwise operators truncate to a signed 32 bit number
  // because JS is a spiteful language.
  // We use BigInts instead, which is wasteful but ergonomic.
  // ᖍ(∙⟞∙)ᖌ
  // https://stackoverflow.com/q/33137519/1163020
  const MAX_IDX = 36n;

  // number of rows (and cols) in the brand
  const cellRows = 6;

  // these are in px
  const brandX = 88; // he's been using brand x!
  const brandY = 24;
  const cellSize = 8;
  const brandSize = cellSize * cellRows;

  const posToCoord = (position) => {
    const pos = Number(position);
    const cx = cellRows - 1 - Math.floor(pos / cellRows);
    const cy = pos % cellRows;
    return {cx, cy}
  }

  const clear = () => {
    context.fillStyle = $pagePalette.light;
    context.fillRect(brandX, brandY, brandSize, brandSize);
  }

  const drawBit = (set, x, y) => {
    context.fillStyle = set ? $pagePalette.white : $pagePalette.dark;
    context.fillRect(x, y, 1, 1);
  }

  const drawChar = (chr, position) => {
    const pattern = chr ? chr.charCodeAt(0) : 0;
    const {cx, cy} = posToCoord(position);

    const x = brandX + cx * cellSize;
    const y = brandY + cy * cellSize;
    // clear area
    context.fillStyle = $pagePalette.light;
    context.fillRect(x, y, 7, 7);
    // surrounding bevel
    context.fillStyle = $pagePalette.white;
    context.fillRect(x + 6, y, 1, 7);
    context.fillRect(x, y + 6, 7, 1);
    context.fillStyle = $pagePalette.dark;
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

  // https://stackoverflow.com/a/5624139/1163020
  const hexToRgb = (hex) => {
    // Expand shorthand form (e.g. "03F") to full form (e.g. "0033FF")
    let shorthandRegex = /^#?([a-f\d])([a-f\d])([a-f\d])$/i;
    hex = hex.replace(shorthandRegex, (m, r, g, b) => r + r + g + g + b + b);

    let result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result ? {
      r: parseInt(result[1], 16),
      g: parseInt(result[2], 16),
      b: parseInt(result[3], 16)
    } : null;
  }

  const recolorBaseImage = () => {
    let imageData = context.getImageData(0, 0, canvas.width, canvas.height);
    let data = imageData.data;
    let lookup = {
      0x00: hexToRgb($pagePalette.black),
      0x80: hexToRgb($pagePalette.dark),
      0xC0: hexToRgb($pagePalette.light),
      0xFF: hexToRgb($pagePalette.white),
    };
    for (let i = 0; i < data.length; i += 4) {
      let {r, g, b} = lookup[grayImage[i]];
      data[i] = r;
      data[i+1] = g;
      data[i+2] = b;
    }
    context.putImageData(imageData, 0, 0);
  }

  const redraw = (text, brandPattern) => {
    if (!context || !grayImage) return;
    clear();
    recolorBaseImage();

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

  const absToCanvas = (evt) => {
    const rect = canvas.getBoundingClientRect();
    return {
      x: Math.floor((evt.clientX - rect.left) / scale),
      y: Math.floor((evt.clientY - rect.top) / scale),
    };
  }

  const handleMousemove = (evt) => {
    const offset = absToCanvas(evt);
    // console.log(offset);
  }

  const handleClick = (evt) => {
    let {x, y} = absToCanvas(evt);
    x -= brandX;
    y -= brandY;
    if (0 <= x && x < brandSize && 0 <= y && y < brandSize) {
      const cx = cellRows - 1 - Math.floor(x / cellSize);
      const cy = Math.floor(y / cellSize);
      const pos = BigInt(cx * cellRows + cy);
      brandPattern ^= 1n << pos;
    }
  }

  $: redraw(text, brandPattern, $pagePalette);

  onMount(() => {
    context = canvas.getContext('2d');
    const image = new Image();
    image.src = brandMural;

    image.onload = () => {
      context.drawImage(image, 0, 0);
      context.lineWidth = 1;
      grayImage = context.getImageData(0, 0, canvas.width, canvas.height).data;
      redraw(text, brandPattern, $pagePalette);
    }
  })
</script>

<div>
  <canvas
    width="224px"
    height="144px"
    bind:this={canvas}
    on:mousemove={handleMousemove}
    on:click={handleClick}
    style="--scale: {scale};"
  />
</div>

<style>
  canvas {
    image-rendering: pixelated;
    width: calc(var(--scale) * 224px);
  }
</style>
