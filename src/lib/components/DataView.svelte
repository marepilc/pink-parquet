<script lang="ts">
  import { dataStore } from '$lib/stores/dataStore.svelte'

  $: data = dataStore.data
  $: loading = dataStore.loading
  $: error = dataStore.error
</script>

<div class="data-view-container">
  {#if loading}
    <div class="center-message">
      <div class="loading-text">Loading Parquet file...</div>
    </div>
  {:else if error}
    <div class="center-message">
      <div class="error-text">Error: {error}</div>
    </div>
  {:else if data}
    <div class="data-sections">
      <!-- File Info -->
      <div class="data-section">
        <h2 class="section-title">File Information</h2>
        <div class="info-grid">
          <div class="info-label">Rows:</div>
          <div class="info-value">{data.shape[0].toLocaleString()}</div>
          <div class="info-label">Columns:</div>
          <div class="info-value">{data.shape[1]}</div>
        </div>
      </div>

      <!-- Columns -->
      <div class="data-section">
        <h2 class="section-title">Columns</h2>
        <div class="columns-list">
          {#each data.columns as column}
            <div class="column-item">
              <span class="column-name">{column.name}</span>
              <span class="column-dtype">{column.dtype}</span>
            </div>
          {/each}
        </div>
      </div>

      <!-- Data Preview -->
      <div class="data-section">
        <h2 class="section-title">
          Data Preview (first {data.rows.length} rows)
        </h2>
        <div class="table-wrapper">
          <table class="data-table">
            <thead>
              <tr class="table-header-row">
                {#each data.columns as column}
                  <th class="table-header">{column.name}</th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each data.rows as row}
                <tr class="table-row">
                  {#each row as cell}
                    <td class="table-cell">{cell}</td>
                  {/each}
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  {:else}
    <div class="center-message">
      <div class="no-data-text">No data loaded</div>
    </div>
  {/if}
</div>

<style>
  .data-view-container {
    display: flex;
    height: 100%;
    width: 100%;
    flex-direction: column;
    overflow: auto;
    padding: 1.5rem;
  }

  .center-message {
    display: flex;
    height: 100%;
    align-items: center;
    justify-content: center;
  }

  .loading-text,
  .no-data-text {
    font-size: 1.125rem;
    color: var(--surface-4);
  }

  .error-text {
    font-size: 1.125rem;
    color: var(--color-error);
  }

  .data-sections {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .data-section {
    background: var(--surface-2);
    border: 1px solid var(--surface-3);
    border-radius: 0.5rem;
    padding: 1rem;
  }

  .section-title {
    color: var(--accent);
    margin-bottom: 0.75rem;
    font-size: 1.25rem;
    font-weight: 600;
  }

  .info-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.5rem;
    font-size: 0.875rem;
  }

  .info-label {
    color: var(--surface-4);
  }

  .info-value {
    color: var(--surface-5);
    font-weight: 500;
  }

  .columns-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .column-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--surface-3);
    border-radius: 0.25rem;
    padding: 0.5rem;
  }

  .column-name {
    color: var(--surface-5);
    font-weight: 500;
  }

  .column-dtype {
    color: var(--surface-4);
    font-family: var(--font-mono), monospace;
    font-size: 0.875rem;
  }

  .table-wrapper {
    overflow-x: auto;
  }

  .data-table {
    width: 100%;
    font-size: 0.875rem;
  }

  .table-header-row {
    border-bottom: 1px solid var(--surface-3);
  }

  .table-header {
    color: var(--surface-5);
    padding: 0.5rem;
    text-align: left;
    font-weight: 600;
  }

  .table-row {
    border-bottom: 1px solid oklch(from var(--surface-3) l c h / 0.5);
  }

  .table-row:hover {
    background: oklch(from var(--surface-3) l c h / 0.3);
  }

  .table-cell {
    color: var(--surface-4);
    padding: 0.5rem;
  }
</style>
