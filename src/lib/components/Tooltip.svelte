<script lang="ts">
  import { tooltipStore } from '$lib/stores/tooltipStore.svelte'

  let tooltipElement = $state<HTMLElement | null>(null)
  let adjustedX = $state(0)

  $effect(() => {
    if (tooltipStore.visible && tooltipElement) {
      const rect = tooltipElement.getBoundingClientRect()
      const windowWidth = window.innerWidth
      const padding = 10 // Minimum distance from the right edge

      // tooltipStore.x is the base position (mouse position)
      // .tooltip has transform: translateX(10px) in CSS
      const currentX = tooltipStore.x + 10

      if (currentX + rect.width > windowWidth - padding) {
        // Shift left to fit within window
        adjustedX = windowWidth - rect.width - padding - 10 // -10 to account for translateX
      } else {
        adjustedX = tooltipStore.x
      }
    }
  })
</script>

{#if tooltipStore.visible}
  <div
    bind:this={tooltipElement}
    class="tooltip"
    style="left: {adjustedX}px; top: {tooltipStore.y}px;"
    role="tooltip"
  >
    {tooltipStore.text}
  </div>
{/if}

<style>
  .tooltip {
    position: fixed;
    z-index: 10000;
    padding: 0.375rem 0.625rem;
    background: var(--surface-7);
    color: var(--ink-1);
    border: 1px solid var(--surface-5);
    border-radius: 0.25rem;
    font-size: var(--text-sm);
    white-space: nowrap;
    pointer-events: none;
    transform: translateX(10px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    max-width: 300px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  :global(.light) .tooltip {
    background: var(--surface-2);
    border-color: var(--surface-5);
    color: #151315;
  }

  :global(.dark) .tooltip {
    background: var(--surface-8);
    border-color: var(--surface-6);
    color: #ECECEC;
  }
</style>
