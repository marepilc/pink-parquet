<script lang="ts">
    import {invoke} from '@tauri-apps/api/core'
    import {onMount} from 'svelte'
    import {dataStore} from '$lib/stores/dataStore.svelte'
    import {tooltipStore} from '$lib/stores/tooltipStore.svelte'

    let engineVersion = $state<string | null>(null)

    const columns = $derived(dataStore.data?.columns || [])
    const totalRows = $derived(dataStore.totalRows)
    const loadedRows = $derived(dataStore.loadedRows)

    const sourceText = $derived.by(() => {
        const session = dataStore.activeSession
        if (!dataStore.hasData || !session) {
            return 'No file loaded'
        }
        if (session.path) {
            return session.path
        }
        return session.isQueryResult ? 'Query result' : session.name
    })

    onMount(async () => {
        try {
            engineVersion = await invoke<string>('engine_version')
        } catch (error) {
            console.error('Failed to read engine version:', error)
        }
    })

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

<footer class="status-bar">
    <span
            class="source"
            role="status"
            onmouseenter={(e) => {
                if (dataStore.activeSession) tooltipStore.show(e.currentTarget, sourceText, e.clientX, e.clientY)
            }}
            onmouseleave={() => tooltipStore.hide()}
    >{sourceText}</span>

    {#if dataStore.data}
        <span>{totalRows.toLocaleString()} rows × {columns.length} columns</span>
        {#if dataStore.metadata && !dataStore.isSqlTabActive}
            <span>{formatFileSize(dataStore.metadata.size)}</span>
        {/if}
        {#if dataStore.metadata}
            <span>data health {formatDataHealth()}</span>
            <span>{dataStore.metadata.compression}</span>
        {/if}
        <span>loaded {loadedRows.toLocaleString()} of {totalRows.toLocaleString()}</span>
    {/if}

    <span class="spacer"></span>
    {#if engineVersion}
        <span class="engine">DuckDB {engineVersion}</span>
    {/if}
</footer>

<style>
    .status-bar {
        display: flex;
        align-items: center;
        gap: 1rem;
        height: 100%;
        min-width: 0;
        padding: 0 0.875rem;
        border-top: 1px solid var(--border);
        background: var(--panel);
        color: var(--text-muted);
        font-size: var(--text-xs);
        white-space: nowrap;
        user-select: none;
    }

    .source {
        min-width: 0;
        flex: 0 1 auto;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .spacer {
        flex: 1;
    }

    .engine {
        font-family: var(--font-mono), monospace;
        color: var(--text-dim);
    }
</style>
