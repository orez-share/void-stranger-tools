<script>
  import GridCell, { CellState } from './GridCell.svelte';
  export let size = "12px";
  export let pattern = null;
  export let palette;
  export let disabled = false;
  export let patternRequirement = null;
  export let faded = false;

  if (pattern != null) pattern = +pattern;

  const sizeStyle = `--size: ${size};`;
  let style;
  $: style = palette + sizeStyle;
  const patternBit = idx => {
    if (pattern == null) return null;
    return pattern & (1 << idx) ? CellState.filled : CellState.void;
  }
  const updateRequirement = bitIdx => evt => {
    const bit = 1 << bitIdx;
    switch (evt.detail.fill) {
      case CellState.unknown:
        patternRequirement.ones &= ~bit;
        patternRequirement.zeroes &= ~bit;
        break;
      case CellState.void:
        patternRequirement.ones &= ~bit;
        patternRequirement.zeroes |= bit;
        break;
      case CellState.filled:
        patternRequirement.ones |= bit;
        patternRequirement.zeroes &= ~bit;
        break;
    }
  }
</script>

<div class="letter-draw-grid" class:faded {style}>
  <GridCell on:update={updateRequirement(6)} {disabled} {faded} {style} state={patternBit(6)} />
  <GridCell on:update={updateRequirement(7)} {disabled} {faded} {style} state={patternBit(7)} />
  <GridCell on:update={updateRequirement(5)} {disabled} {faded} {style} state={patternBit(5)} />
  <GridCell on:update={updateRequirement(4)} {disabled} {faded} {style} state={patternBit(4)} />
  <div></div>
  <GridCell on:update={updateRequirement(3)} {disabled} {faded} {style} state={patternBit(3)} />
  <GridCell on:update={updateRequirement(2)} {disabled} {faded} {style} state={patternBit(2)} />
  <GridCell on:update={updateRequirement(0)} {disabled} {faded} {style} state={patternBit(0)} />
  <GridCell on:update={updateRequirement(1)} {disabled} {faded} {style} state={patternBit(1)} />
</div>

<style>
  .letter-draw-grid {
    display: inline-grid;
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

  .letter-draw-grid.faded {
    background-color: var(--color-dark);
    border-top-color: var(--color-black);
    border-left-color: var(--color-black);
    border-bottom-color: var(--color-light);
    border-right-color: var(--color-light);
    color: var(--color-black);
  }
</style>
