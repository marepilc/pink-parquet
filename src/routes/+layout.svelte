<script lang="ts">
    import {invoke} from '@tauri-apps/api/core'
    import '../app.css'
    import TitleBar from '$lib/components/TitleBar.svelte'
    import Sidebar from '$lib/components/Sidebar.svelte'
    import StatusBar from '$lib/components/StatusBar.svelte'
    import DragOverlay from '$lib/components/DragOverlay.svelte'
    import Tooltip from '$lib/components/Tooltip.svelte'
    import type {Snippet} from 'svelte'
    import {onDestroy, onMount} from 'svelte'
    import {dataStore} from '$lib/stores/dataStore.svelte'
    import {settingsStore} from '$lib/stores/settingsStore.svelte'
    import {goto} from '$app/navigation'
    import type {UnlistenFn} from '@tauri-apps/api/event'
    import {loadDataFile, openDataFiles, saveActiveAs} from '$lib/fileActions'
    import {type} from '@tauri-apps/plugin-os'

    interface Props {
        children: Snippet
    }

    let {children}: Props = $props()

    let unlistenDrag: UnlistenFn | null = null
    let unlistenOpenFile: UnlistenFn | null = null
    let unlistenFileChanged: UnlistenFn | null = null
    let isDraggingValidFile = $state(false)
    let draggedFileExtension = $state('')
    let isMacOS = $state(false)
    let isLinux = $state(false)

    function isValidFileType(filePath: string): boolean {
        const lower = filePath.toLowerCase()
        return lower.endsWith('.parquet') || lower.endsWith('.csv') || lower.endsWith('.sql')
    }

    function isDataFile(filePath: string): boolean {
        const lower = filePath.toLowerCase()
        return lower.endsWith('.parquet') || lower.endsWith('.csv')
    }

    function isSqlFile(filePath: string): boolean {
        return filePath.toLowerCase().endsWith('.sql')
    }

    async function loadSqlFile(filePath: string) {
        try {
            const content = await invoke<string>('read_text_file', {path: filePath})
            if (content) {
                // Ensure there is an active session to load SQL into
                if (!dataStore.activeSessionId && dataStore.sessions.length > 0) {
                    dataStore.activeSessionId = dataStore.sessions[0].id
                }

                if (dataStore.activeSessionId) {
                    dataStore.setQuery(content)
                    dataStore.resetQueryResults()
                    dataStore.isSqlTabActive = true
                    await goto('/app')
                } else {
                    console.warn('No active session to load SQL file into')
                }
            }
        } catch (error) {
            console.error('Error loading SQL file:', error)
        }
    }

    async function loadParquetFile(filePath: string, forceReload: boolean = false) {
        await loadDataFile(filePath, forceReload)
    }

    function handleKeyDown(event: KeyboardEvent) {
        // Check for Ctrl (Windows/Linux) or Cmd (Mac)
        const isModifier = event.ctrlKey || event.metaKey

        if (isModifier && event.key === 'o') {
            event.preventDefault()
            openDataFiles()
        } else if (isModifier && event.key === 's') {
            event.preventDefault()
            saveActiveAs()
        } else if (event.key === 'F5' || (event.shiftKey && event.key === 'Enter')) {
            // F5 or Shift + Enter is handled by SqlEditor if focused.
            // If we're here, it means focus is elsewhere.
            // We prevent default to avoid page reload which clears state.
            if (dataStore.isSqlTabActive) {
                event.preventDefault()
                // Only trigger if focus is NOT in an input/textarea (which SqlEditor uses)
                // Actually CodeMirror handles its own events, but let's be safe.
                const target = event.target as HTMLElement
                if (target.tagName !== 'TEXTAREA' && target.tagName !== 'INPUT') {
                    dataStore.triggerQuery()
                }
            }
        }
    }

    function handleContextMenu(event: MouseEvent) {
        // Prevent default browser context menu globally
        event.preventDefault()
    }

    onMount(async () => {
        const {listen} = await import('@tauri-apps/api/event')
        const {getCurrentWebview} = await import('@tauri-apps/api/webview')

        // Detect OS
        const osType = await type()
        isMacOS = osType === 'macos'
        isLinux = osType === 'linux'

        // Initialize settings
        await settingsStore.init()

        // Check for updates
        dataStore.checkUpdates()

        unlistenOpenFile = await listen<string>('open-file', (event) => {
            const filePath = event.payload
            if (isDataFile(filePath)) {
                loadParquetFile(filePath)
            }
        })

        unlistenFileChanged = await listen<string>('file-changed', async (event) => {
            const filePath = event.payload
            // Reload the file
            await loadParquetFile(filePath, true)
        })

        unlistenDrag = await getCurrentWebview().onDragDropEvent((event) => {
            const payload = event.payload as { type: string; paths?: string[] }

            if (payload.type === 'enter' || payload.type === 'over') {
                const paths = payload.paths
                if (paths && paths.length > 0) {
                    const filePath = paths[0]
                    if (isValidFileType(filePath)) {
                        isDraggingValidFile = true
                        draggedFileExtension = filePath.split('.').pop() || ''
                    }
                }
            } else if (payload.type === 'drop') {
                isDraggingValidFile = false
                const paths = payload.paths

                if (paths && paths.length > 0) {
                    const filePath = paths[0]
                    if (isDataFile(filePath)) {
                        loadParquetFile(filePath)
                    } else if (isSqlFile(filePath)) {
                        loadSqlFile(filePath)
                    }
                }
            } else if (payload.type === 'leave' || payload.type === 'cancel') {
                isDraggingValidFile = false
                draggedFileExtension = ''
            }
        })
    })

    onDestroy(() => {
        if (unlistenDrag) unlistenDrag()
        if (unlistenOpenFile) unlistenOpenFile()
        if (unlistenFileChanged) unlistenFileChanged()
    })
</script>

<svelte:window onkeydown={handleKeyDown} oncontextmenu={handleContextMenu}/>

<main class="main-window" class:macos={isMacOS} class:linux={!isMacOS && isLinux}>
    {#if !isMacOS && !isLinux}
        <header id="title-bar-panel">
            <TitleBar/>
        </header>
    {/if}
    <div id="sidebar">
        <Sidebar/>
    </div>
    <div id="main-area">{@render children()}</div>
    <div id="status-bar">
        <StatusBar/>
    </div>

    <DragOverlay
            isVisible={isDraggingValidFile}
            hasData={dataStore.hasData}
            fileExtension={draggedFileExtension}
    />
    <Tooltip/>
</main>

<style>
    main {
        width: 100vw;
        height: 100vh;
        display: grid;
        grid-template-rows: 2.5rem 1fr 1.75rem;
        grid-template-columns: 15rem 1fr;
        grid-template-areas:
      'title-bar title-bar'
      'sidebar main-area'
      'status-bar status-bar';
        overflow: hidden;
        background: var(--bg);
        color: var(--text);
    }

    main.macos,
    main.linux {
        grid-template-rows: 1fr 1.75rem;
        grid-template-areas:
      'sidebar main-area'
      'status-bar status-bar';
    }

    #title-bar-panel {
        width: 100%;
        grid-area: title-bar;
        display: flex;
        align-items: center;
        justify-content: space-between;
        z-index: 100;
        user-select: none;
        background: var(--panel);
        border-bottom: 1px solid var(--border);
    }

    #sidebar {
        grid-area: sidebar;
        min-height: 0;
    }

    #main-area {
        grid-area: main-area;
        overflow: hidden;
        min-width: 0;
        display: flex;
        flex-direction: column;
    }

    #status-bar {
        grid-area: status-bar;
        min-width: 0;
    }
</style>
