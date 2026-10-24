<script lang="ts">
    import {dataStore, type FileSession} from '$lib/stores/dataStore.svelte'
    import {settingsStore} from '$lib/stores/settingsStore.svelte'
    import {fontStore} from '$lib/stores/fontStore.svelte'
    import {openDataFiles, saveActiveAs} from '$lib/fileActions'
    import ToggleSwitch from '$lib/components/ToggleSwitch.svelte'
    import AboutModal from '$lib/components/AboutModal.svelte'
    import SqlIcon from '$lib/components/icons/SqlIcon.svelte'
    import ParquetIcon from '$lib/components/icons/ParquetIcon.svelte'
    import PlusIcon from '$lib/components/icons/PlusIcon.svelte'
    import MinusIcon from '$lib/components/icons/MinusIcon.svelte'

    let showAboutModal = $state(false)

    const fileSessions = $derived(dataStore.sessions.filter((s) => !s.isQueryEditor && !s.isQueryResult))
    const querySessions = $derived(dataStore.sessions.filter((s) => s.isQueryEditor || s.isQueryResult))
    const isMinFontSize = $derived(fontStore.currentFontSize <= 12)
    const isMaxFontSize = $derived(fontStore.currentFontSize >= 18)

    function activate(session: FileSession) {
        dataStore.activeSessionId = session.id
        dataStore.isSqlTabActive = !!session.isQueryEditor
    }

    async function handleThemeChange(isDark: boolean) {
        await settingsStore.setTheme(isDark ? 'dark' : 'light')
    }
</script>

<nav class="sidebar" aria-label="Data sources">
    <div class="actions">
        <button type="button" class="action primary" onclick={openDataFiles}>Open file…</button>
        <button
                type="button"
                class="action"
                onclick={() => dataStore.addQueryEditorSession()}
                disabled={!dataStore.hasData}
        >New SQL
        </button>
        <button type="button" class="action" onclick={saveActiveAs} disabled={!dataStore.data}>Save as…</button>
    </div>

    <div class="sections">
        <section>
            <div class="section-header">
                <h2>Files</h2>
                <button type="button" class="icon-btn" aria-label="Open file" onclick={openDataFiles}>
                    <PlusIcon size={12}/>
                </button>
            </div>
            {#each fileSessions as session (session.id)}
                <button
                        type="button"
                        class="item"
                        class:active={dataStore.activeSessionId === session.id}
                        title={session.path ?? session.name}
                        onclick={() => activate(session)}
                >
                    <ParquetIcon size={14}/>
                    <span class="item-name">{session.name}</span>
                </button>
            {:else}
                <p class="empty">No files open</p>
            {/each}
        </section>

        {#if querySessions.length > 0}
            <section>
                <div class="section-header">
                    <h2>Queries</h2>
                </div>
                {#each querySessions as session (session.id)}
                    <button
                            type="button"
                            class="item"
                            class:active={dataStore.activeSessionId === session.id}
                            onclick={() => activate(session)}
                    >
                        <SqlIcon size={14}/>
                        <span class="item-name">{session.name}</span>
                    </button>
                {/each}
            </section>
        {/if}

        <section>
            <div class="section-header">
                <h2>Connections</h2>
            </div>
            <p class="empty">DuckDB, Delta Lake and PostgreSQL are coming in v3.</p>
        </section>
    </div>

    <div class="settings">
        <div class="setting-row">
            <button
                    type="button"
                    class="font-btn"
                    onclick={() => fontStore.cycleFont()}
                    aria-label="Change table font"
            >{fontStore.currentFont}</button>
            <div class="font-size">
                <button
                        type="button"
                        class="icon-btn"
                        aria-label="Decrease font size"
                        onclick={() => fontStore.decreaseFontSize()}
                        disabled={isMinFontSize}
                >
                    <MinusIcon size={12}/>
                </button>
                <span class="font-size-value">{fontStore.currentFontSize}</span>
                <button
                        type="button"
                        class="icon-btn"
                        aria-label="Increase font size"
                        onclick={() => fontStore.increaseFontSize()}
                        disabled={isMaxFontSize}
                >
                    <PlusIcon size={12}/>
                </button>
            </div>
        </div>
        <div class="setting-row">
            <span class="setting-label">Dark theme</span>
            <ToggleSwitch
                    checked={settingsStore.settings.theme === 'dark'}
                    onChange={handleThemeChange}
                    ariaLabel="Toggle dark mode"
            />
        </div>
        <button type="button" class="about-btn" onclick={() => (showAboutModal = true)}>
            About Pink Parquet
            {#if dataStore.updateCount > 0 && !dataStore.updateSeen}
                <span class="update-dot" aria-label="Update available"></span>
            {/if}
        </button>
    </div>
</nav>

<AboutModal bind:isOpen={showAboutModal}/>

<style>
    .sidebar {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        height: 100%;
        min-height: 0;
        padding: 0.75rem 0.625rem;
        background: var(--panel);
        border-right: 1px solid var(--border);
        user-select: none;
    }

    .actions {
        display: flex;
        flex-wrap: wrap;
        gap: 0.375rem;
    }

    .action {
        flex: 1 1 auto;
        height: 1.875rem;
        padding: 0 0.625rem;
        border-radius: 0.4375rem;
        border: 1px solid var(--border-strong);
        background: none;
        color: var(--text-2);
        font: inherit;
        font-size: var(--text-sm);
        cursor: pointer;

        &:hover:not(:disabled) {
            background: var(--hover);
            color: var(--text);
        }

        &:disabled {
            opacity: 0.45;
            cursor: default;
        }
    }

    .action.primary {
        flex-basis: 100%;
        border: 0;
        background: var(--accent);
        color: var(--accent-fg);
        font-weight: 600;

        &:hover {
            background: var(--accent);
            color: var(--accent-fg);
            filter: brightness(1.08);
        }
    }

    .sections {
        flex: 1 1 auto;
        min-height: 0;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
    }

    section {
        display: flex;
        flex-direction: column;
        gap: 0.125rem;
    }

    .section-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0 0.375rem 0.375rem;

        h2 {
            margin: 0;
            font-size: var(--text-xs);
            font-weight: 600;
            letter-spacing: 0.08em;
            text-transform: uppercase;
            color: var(--text-dim);
        }
    }

    .item {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        min-height: 1.875rem;
        padding: 0.3125rem 0.375rem;
        border: 0;
        border-radius: 0.375rem;
        background: none;
        color: var(--text-2);
        font: inherit;
        font-size: var(--text-base);
        text-align: left;
        cursor: pointer;

        &:hover {
            background: var(--hover);
            color: var(--text);
        }

        &.active {
            background: var(--hover);
            color: var(--text);
            box-shadow: inset 2px 0 0 var(--accent);
        }
    }

    .item-name {
        flex: 1;
        min-width: 0;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .empty {
        margin: 0;
        padding: 0.25rem 0.375rem;
        font-size: var(--text-sm);
        color: var(--text-dim);
        line-height: 1.45;
    }

    .icon-btn {
        width: 1.5rem;
        height: 1.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        border: 0;
        border-radius: 0.375rem;
        background: var(--chip);
        color: var(--text-2);
        cursor: pointer;

        &:hover:not(:disabled) {
            color: var(--text);
            background: var(--hover);
        }

        &:disabled {
            opacity: 0.35;
            cursor: default;
        }
    }

    .settings {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        padding-top: 0.75rem;
        border-top: 1px solid var(--border);
    }

    .setting-row {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.5rem;
        padding: 0 0.375rem;
    }

    .setting-label {
        font-size: var(--text-sm);
        color: var(--text-muted);
    }

    .font-btn {
        height: 1.625rem;
        padding: 0 0.5rem;
        border-radius: 0.375rem;
        border: 1px solid var(--border-strong);
        background: none;
        color: var(--text-2);
        font-family: var(--font-mono), monospace;
        font-size: var(--text-sm);
        cursor: pointer;

        &:hover {
            color: var(--text);
            background: var(--hover);
        }
    }

    .font-size {
        display: flex;
        align-items: center;
        gap: 0.375rem;
    }

    .font-size-value {
        min-width: 1.25rem;
        text-align: center;
        font-size: var(--text-sm);
        color: var(--text-muted);
    }

    .about-btn {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.375rem;
        border: 0;
        border-radius: 0.375rem;
        background: none;
        color: var(--text-muted);
        font: inherit;
        font-size: var(--text-sm);
        text-align: left;
        cursor: pointer;

        &:hover {
            background: var(--hover);
            color: var(--text);
        }
    }

    .update-dot {
        width: 0.4375rem;
        height: 0.4375rem;
        border-radius: 50%;
        background: var(--accent);
    }
</style>
