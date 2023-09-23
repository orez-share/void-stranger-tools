<script>
  import GridCell, { CellState } from './GridCell.svelte';
  export let size = "12px";
  export let pattern = null;
  export let palette;
  export let disabled = false;

  if (pattern != null) pattern = +pattern;

  const sizeStyle = `--size: ${size};`;
  let style;
  $: style = palette + sizeStyle;
  const patternBit = idx => {
    if (pattern == null) return null;
    return pattern & (1 << idx) ? CellState.filled : CellState.void;
  }
</script>

<div class="letter-draw-grid" style={style}>
  <GridCell disabled={disabled} style={style} state={patternBit(0)} />
  <GridCell disabled={disabled} style={style} state={patternBit(1)} />
  <GridCell disabled={disabled} style={style} state={patternBit(2)} />
  <GridCell disabled={disabled} style={style} state={patternBit(3)} />
  <div></div>
  <GridCell disabled={disabled} style={style} state={patternBit(4)} />
  <GridCell disabled={disabled} style={style} state={patternBit(5)} />
  <GridCell disabled={disabled} style={style} state={patternBit(6)} />
  <GridCell disabled={disabled} style={style} state={patternBit(7)} />
</div>

<style>
  .letter-draw-grid {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    width: max-content;
    border-style: solid;

    grid-gap: var(--size);
    border-width: var(--size);

    background-color: var(--color-light);
    border-top-color: var(--color-dark);
    border-left-color: var(--color-dark);
    border-bottom-color: var(--color-white);
    border-right-color: var(--color-white);
    color: var(--color-black);
  }
</style>
