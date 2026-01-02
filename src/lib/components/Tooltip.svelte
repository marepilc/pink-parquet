<script lang="ts">
  import { tooltipStore } from '$lib/stores/tooltipStore.svelte'
</script>

{#if tooltipStore.visible}
  <div
    class="tooltip"
    style="left: {tooltipStore.x}px; top: {tooltipStore.y}px;"
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
    transform: translateX(-50%);
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
