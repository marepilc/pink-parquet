<script lang="ts">
    import {dataStore} from '$lib/stores/dataStore.svelte'

    const columns = $derived(dataStore.data?.columns || [])
    const totalRows = $derived(dataStore.totalRows)
    const loadedRows = $derived(dataStore.loadedRows)

    function formatDataHealth(): string {
        if (!dataStore.metadata) return '-'

        const totalCells = totalRows * columns.length
        if (totalCells === 0) return '0%'

        const totalNulls = dataStore.metadata.total_nulls || 0
        const filledCells = totalCells - totalNulls
        const healthPercent = (filledCells / totalCells) * 100

        // Handle edge cases
        if (healthPercent > 0 && healthPercent < 0.05) {
            return '~0%'
        }
        if (healthPercent > 99.95 && healthPercent < 100) {
            return '~100%'
        }

        // Format with 1 decimal if not whole number
        if (healthPercent % 1 === 0) {
            return `${healthPercent.toFixed(0)}%`
        }
        return `${healthPercent.toFixed(1)}%`
    }

    function formatFileSize(bytes: number): string {
        if (bytes === 0) return '0 B'
        const k = 1024
        const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
        const i = Math.floor(Math.log(bytes) / Math.log(k))
        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
    }
</script>

{#if dataStore.metadata}
    <div id="info-bar-container">
        <div class="info-bar-content">
            <div class="info-metrics">
                <div class="info-metric">
                    <span class="metric-label">Rows:</span>
                    <span class="metric-value">{totalRows.toLocaleString()}</span>
                </div>
                <div class="info-metric">
                    <span class="metric-label">Columns:</span>
                    <span class="metric-value">{columns.length}</span>
                </div>
                {#if !dataStore.isSqlTabActive}
                    <div class="info-metric">
                        <span class="metric-label">Size:</span>
                        <span class="metric-value">{formatFileSize(dataStore.metadata.size)}</span>
                    </div>
                {/if}
                <div class="info-metric">
                    <span class="metric-label">Data health:</span>
                    <span class="metric-value">{formatDataHealth()}</span>
                </div>
                <div class="info-metric">
                    <span class="metric-label">Compression:</span>
                    <span class="metric-value">{dataStore.metadata.compression}</span>
                </div>
            </div>
            <div class="loaded-rows">
                Loaded {loadedRows.toLocaleString()} of {totalRows.toLocaleString()} rows
            </div>
        </div>
    </div>
{/if}

<style>
    #info-bar-container {
        display: flex;
        flex-grow: 1;
        padding: 0 0.5rem;
        user-select: none;
    }

    .info-bar-content {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
    }

    .info-metrics {
        display: flex;
        align-items: center;
        gap: 1.5rem;
        font-size: var(--text-xs);
    }

    .info-metric {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .metric-label {
        color: var(--color-ink-5);
        font-size: var(--text-xs);
    }

    .metric-value {
        color: var(--color-ink-1);
        font-weight: 500;
    }

    .loaded-rows {
        color: var(--color-ink-5);
        font-size: var(--text-xs);
        margin-left: auto;
    }
</style>
