<script lang="ts">
  import { dataStore } from '$lib/stores/dataStore.svelte'

  function sanitizeTableName(name: string): string {
    let sanitized = name.replace(/[^A-Za-z0-9_]/g, '_')
    if (/^[0-9]/.test(sanitized)) {
      sanitized = '_' + sanitized
    }
    return sanitized || 'table'
  }

  const availableTables = $derived(
    dataStore.sessions.map((session) => ({
      tableName: sanitizeTableName(session.name),
      columns: session.baseColumns || session.rawData?.columns || [],
    }))
  )
</script>

<div class="guide-container">
  <div class="guide-content">
    <div class="guide-section">
      <h3 class="section-title">Available Tables</h3>
      {#if availableTables.length === 0}
        <p class="section-text">
          No tables loaded. Open a Parquet file to get started.
        </p>
      {:else}
        {#each availableTables as table}
          <div class="table-info">
            <div class="table-name">
              <code class="inline-code">{table.tableName}</code>
            </div>
            {#if table.columns.length > 0}
              <div class="columns-list">
                {#each table.columns as column}
                  <div class="column-item">
                    <span class="column-name">{column.name}</span>
                    <span class="column-type">{column.dtype}</span>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="no-columns">Loading columns...</p>
            {/if}
          </div>
        {/each}
      {/if}
    </div>

    <div class="guide-section">
      <h3 class="section-title">Keyboard Shortcuts</h3>
      <div class="shortcuts">
        <div class="shortcut-item">
          <kbd class="kbd">F5</kbd>
          <span class="shortcut-desc">Execute query</span>
        </div>
        <div class="shortcut-item">
          <kbd class="kbd">Ctrl</kbd> + <kbd class="kbd">Space</kbd>
          <span class="shortcut-desc">Show autocomplete</span>
        </div>
      </div>
    </div>

    <div class="guide-footer">
      <p class="footer-text">
        💡 SQL keywords are automatically capitalized as you type
      </p>
    </div>
  </div>
</div>

<style>
  .guide-container {
    display: flex;
    align-items: flex-start;
    justify-content: center;
    height: 100%;
    overflow-y: auto;
    padding: 2rem;
    background: oklch(from var(--surface-2) l c h / 0.75);
  }

  .guide-content {
    max-width: 48rem;
    width: 100%;
    margin: auto 0;
  }

  .guide-section {
    margin-bottom: 2rem;
  }

  .section-title {
    font-size: var(--text-lg);
    font-weight: 600;
    color: var(--ink-5);
    margin: 0 0 0.75rem 0;
  }

  .section-text {
    font-size: var(--text-base);
    color: var(--ink-4);
    line-height: 1.6;
    margin: 0;
  }

  .inline-code {
    background: var(--surface-3);
    color: var(--accent);
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-family: var(--font-mono), monospace;
    font-size: 0.9em;
  }

  .table-info {
    background: var(--surface-2);
    border: 1px solid var(--surface-4);
    border-radius: 0.375rem;
    padding: 1rem;
    margin-bottom: 1rem;
  }

  .table-info:last-child {
    margin-bottom: 0;
  }

  .table-name {
    margin-bottom: 0.75rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--surface-4);
  }

  .columns-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .column-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.375rem 0.5rem;
    background: var(--surface-1);
    border-radius: 0.25rem;
    font-size: var(--text-sm);
  }

  .column-name {
    font-family: var(--font-mono), monospace;
    color: var(--ink-5);
    font-weight: 500;
  }

  .column-type {
    font-family: var(--font-mono), monospace;
    color: var(--ink-3);
    font-size: var(--text-xs);
    background: var(--surface-3);
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
  }

  .no-columns {
    font-size: var(--text-sm);
    color: var(--ink-3);
    font-style: italic;
    margin: 0;
  }

  .shortcuts {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .shortcut-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .kbd {
    display: inline-block;
    padding: 0.25rem 0.5rem;
    background: var(--surface-2);
    border: 1px solid var(--surface-5);
    border-radius: 0.25rem;
    font-family: var(--font-sans), ui-sans-serif, sans-serif;
    font-size: var(--text-xs);
    font-weight: 600;
    color: var(--ink-5);
    box-shadow: 0 2px 0 var(--surface-6);
  }

  .shortcut-desc {
    font-size: var(--text-sm);
    color: var(--ink-4);
  }

  .guide-footer {
    margin-top: 2rem;
    padding-top: 1rem;
    border-top: 1px solid var(--surface-4);
  }

  .footer-text {
    font-size: var(--text-sm);
    color: var(--ink-3);
    margin: 0;
    text-align: center;
  }
</style>
