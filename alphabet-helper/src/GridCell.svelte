<script context="module">
  export const CellState = {
    void: 0,
    filled: 1,
    unknown: 2,
  };
</script>

<script>
  export let style;
  export let state = CellState.unknown;
  export let disabled = false;

  let fill = state ?? CellState.unknown;
  const cycleFill = () => {
    if (disabled) return;
    fill += 2;
    fill %= 3;
  }
</script>

<div style={style} on:click={cycleFill}
  class="letter-grid-cell"
  class:disabled={disabled}
  class:unknown={fill === CellState.unknown}
  class:filled={fill === CellState.filled}
  class:void={fill === CellState.void}
></div>

<style>
  .letter-grid-cell {
    width: var(--size);
    height: var(--size);
    font-family: Arial;
    user-select: none;
    text-align: center;
    font-size: 40px;
  }

  .letter-grid-cell:not(.disabled) {
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
