<script>
  export let name;
  export let palette;
  export let handleSelect;
  palette = {
    white: "#fff",
    black: "#000",
    ...palette
  };
  const paletteStyle = `
    --color-black: ${palette.black};
    --color-dark: ${palette.dark};
    --color-light: ${palette.light};
    --color-white: ${palette.white};
  `;
</script>

<div on:click={() => handleSelect(palette)} class="palette-outer" style={paletteStyle}>
  <div class="palette-inner" style={paletteStyle}>
  </div>
</div>

<style>
  .palette-outer {
    width: fit-content;
    border-width: 8px;
    border-style: solid;

    border-left-color: var(--color-black);
    border-top-color: var(--color-white);
    border-right-color: var(--color-white);
    border-bottom-color: var(--color-black);
  }

  .palette-inner {
    width: 0px;
    height: 0px;
    border-width: 20px;
    border-style: solid;

    border-left-color: var(--color-light);
    border-top-color: var(--color-light);
    border-right-color: var(--color-dark);
    border-bottom-color: var(--color-dark);
  }
</style>
