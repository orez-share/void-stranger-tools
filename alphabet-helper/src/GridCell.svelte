<script>
  export let paletteStyle;
  const UNKNOWN = 0;
  const FILLED = 1;
  const VOID = 2;

  let fill = 0;
  const cycleFill = () => {
    fill += 1;
    fill %= 3;
  }
</script>

<div style={paletteStyle} on:click={cycleFill}
  class="letter-grid-cell"
  class:unknown={fill === UNKNOWN}
  class:filled={fill === FILLED}
  class:void={fill === VOID}
></div>

<style>
  .letter-grid-cell {
    width: 50px;
    height: 50px;
    font-family: Arial;
    user-select: none;
    text-align: center;
    font-size: 40px;
    cursor: pointer;
  }

  .letter-grid-cell.filled {
    background-color: var(--color-white);
  }

  .letter-grid-cell.void {
    background-color: var(--color-dark);
  }

  .letter-grid-cell.unknown {
    background-color: var(--color-light);
  }

  .letter-grid-cell.unknown::after {
    content: "?";
  }
</style>
