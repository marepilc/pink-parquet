<script lang="ts">
  import DataTable from '$lib/components/DataTable.svelte'
  import QueryView from '$lib/components/query/QueryView.svelte'
  import Tabs from '$lib/components/Tabs.svelte'
  import { dataStore } from '$lib/stores/dataStore.svelte'
  import { goto } from '$app/navigation'
  import { onMount } from 'svelte'

  // Redirect to landing if no sessions (guard for direct navigation)
  onMount(() => {
    if (!dataStore.hasData) {
      goto('/')
    }
  })
</script>

{#if dataStore.hasData}
  <div class="app-container">
    <div class={dataStore.isSqlTabActive ? 'query-visible' : 'query-hidden'}>
      <QueryView visible={dataStore.isSqlTabActive} />
    </div>
    <DataTable />
    <Tabs />
  </div>
{/if}

<style>
  .app-container {
    display: flex;
    height: 100%;
    flex-direction: column;
  }

  /*noinspection CssUnusedSymbol*/
  .query-visible {
    display: contents;
  }

  /*noinspection CssUnusedSymbol*/
  .query-hidden {
    display: none;
  }
</style>
