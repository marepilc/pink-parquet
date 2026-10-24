<script lang="ts">
    import {openDataFiles} from '$lib/fileActions'
</script>

<div class="start">
    <div class="content">
        <div class="heading">
            <h1>What are we querying today?</h1>
            <p>Parquet and CSV files today. DuckDB, Delta Lake and PostgreSQL are coming in v3.</p>
        </div>

        <div class="drop-zone">
            <span class="drop-icon" aria-hidden="true">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                     stroke-linecap="round" stroke-linejoin="round">
                    <path d="M12 15V3"></path>
                    <path d="m7 8 5-5 5 5"></path>
                    <path d="M20 15v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-4"></path>
                </svg>
            </span>
            <div class="drop-text">
                <span class="drop-title">Drop a file anywhere in the window</span>
                <span class="drop-hint">Parquet · CSV · SQL</span>
            </div>
            <button type="button" class="open-btn" onclick={openDataFiles}>
                Open file… <kbd>Ctrl O</kbd>
            </button>
        </div>

        <div class="shortcuts">
            <span><kbd>Ctrl O</kbd> open file</span>
            <span><kbd>Ctrl S</kbd> save as Parquet or CSV</span>
            <span><kbd>F5</kbd> run query</span>
        </div>
    </div>
</div>

<style>
    .start {
        display: flex;
        justify-content: center;
        height: 100%;
        overflow-y: auto;
        padding: 3.5rem 1rem 2rem;
        user-select: none;
    }

    .content {
        width: 100%;
        max-width: 60rem;
        display: flex;
        flex-direction: column;
        gap: 2rem;
    }

    .heading {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;

        h1 {
            margin: 0;
            font-size: var(--text-4xl);
            font-weight: 700;
            letter-spacing: -0.02em;
            color: var(--text);
        }

        p {
            margin: 0;
            font-size: var(--text-lg);
            color: var(--text-muted);
        }
    }

    .drop-zone {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.125rem;
        padding: 2.75rem 1.5rem;
        border-radius: 1rem;
        border: 1.5px dashed var(--border-strong);
        background: var(--panel);
        text-align: center;
    }

    .drop-icon {
        width: 3.25rem;
        height: 3.25rem;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 0.875rem;
        background: var(--accent-soft);
        color: var(--accent-soft-fg);
    }

    .drop-text {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .drop-title {
        font-size: var(--text-lg);
        font-weight: 600;
        color: var(--text);
    }

    .drop-hint {
        color: var(--text-muted);
    }

    .open-btn {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        height: 2.375rem;
        padding: 0 1rem;
        border: 0;
        border-radius: 0.5625rem;
        background: var(--accent);
        color: var(--accent-fg);
        font: inherit;
        font-weight: 600;
        cursor: pointer;

        &:hover {
            filter: brightness(1.08);
        }

        kbd {
            opacity: 0.8;
        }
    }

    .shortcuts {
        display: flex;
        flex-wrap: wrap;
        gap: 0.625rem;

        span {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            height: 2rem;
            padding: 0 0.75rem;
            border-radius: 0.5rem;
            background: var(--chip);
            color: var(--text-muted);
            font-size: var(--text-sm);
        }
    }

    kbd {
        font-family: var(--font-mono), monospace;
        font-size: var(--text-xs);
        color: inherit;
    }

    .shortcuts kbd {
        color: var(--text);
    }
</style>
