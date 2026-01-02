<script lang="ts">
  import { dataStore } from '$lib/stores/dataStore.svelte'
</script>

<footer id="app-footer">
  <div>
    {#if dataStore.hasData && dataStore.activeSession}
      {dataStore.activeSession.path}
    {:else}
      No parquet file loaded.
    {/if}
  </div>
</footer>

<style>
  #app-footer {
    padding: 0.25rem 0.5rem;
    border-top: 1px var(--surface-3) solid;
    display: flex;
    align-items: center;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--ink-3);
    white-space: nowrap;
    font-size: 0.75rem;
    font-style: italic;
    user-select: none;
  }
</style>
