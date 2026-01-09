<script lang="ts">
    import '../app.css'
    import TitleBar from '$lib/components/TitleBar.svelte'
    import Toolbar from '$lib/components/Toolbar.svelte'
    import Footer from '$lib/components/Footer.svelte'
    import DragOverlay from '$lib/components/DragOverlay.svelte'
    import Tooltip from '$lib/components/Tooltip.svelte'
    import type {Snippet} from 'svelte'
    import {onDestroy, onMount} from 'svelte'
    import {getCurrentWebview} from '@tauri-apps/api/webview'
    import {invoke} from '@tauri-apps/api/core'
    import {dataStore} from '$lib/stores/dataStore.svelte'
    import {settingsStore} from '$lib/stores/settingsStore.svelte'
    import {goto} from '$app/navigation'
    import {listen, type UnlistenFn} from '@tauri-apps/api/event'
    import InfoBar from '$lib/components/InfoBar.svelte'
    import {open, save} from '@tauri-apps/plugin-dialog'
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
        return filePath.toLowerCase().endsWith('.parquet') || filePath.toLowerCase().endsWith('.sql')
    }

    function isParquetFile(filePath: string): boolean {
        return filePath.toLowerCase().endsWith('.parquet')
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
        await dataStore.loadParquetFile(filePath, forceReload, goto)
    }

    // Keyboard shortcuts
    async function handleOpenFile() {
        try {
            const files = await open({
                multiple: true,
                filters: [
                    {
                        name: 'Parquet',
                        extensions: ['parquet'],
                    },
                ],
            })

            if (files && Array.isArray(files)) {
                for (const file of files) {
                    await loadParquetFile(file)
                }
            } else if (files) {
                await loadParquetFile(files as string)
            }
        } catch (error) {
            console.error('Error opening file:', error)
        }
    }

    async function handleSaveAs() {
        if (!dataStore.hasData) return

        try {
            let defaultPath: string | undefined

            if (dataStore.isSqlTabActive) {
                defaultPath = 'untitled.parquet'
            } else {
                const activeSession = dataStore.sessions.find(
                    (s) => s.id === dataStore.activeSessionId
                )
                if (activeSession) {
                    const originalPath = activeSession.path
                    const pathSeparator = originalPath.includes('/') ? '/' : '\\'
                    const lastSeparatorIndex = originalPath.lastIndexOf(pathSeparator)

                    if (lastSeparatorIndex >= 0) {
                        const directory = originalPath.substring(0, lastSeparatorIndex + 1)
                        const name = activeSession.name.endsWith('.parquet')
                            ? activeSession.name
                            : `${activeSession.name}.parquet`
                        defaultPath = directory + name
                    } else {
                        defaultPath = activeSession.name.endsWith('.parquet')
                            ? activeSession.name
                            : `${activeSession.name}.parquet`
                    }
                }
            }

            const filePath = await save({
                defaultPath,
                filters: [
                    {
                        name: 'Parquet',
                        extensions: ['parquet'],
                    },
                ],
            })

            if (filePath) {
                await invoke('save_parquet', {filePath})
                console.log('File saved successfully to:', filePath)

                // If the saved file is already open in any session, reload that session
                const existingSession = dataStore.sessions.find((s) => s.path === filePath)
                if (existingSession) {
                    await loadParquetFile(filePath, true)
                }
            }
        } catch (error) {
            console.error('Error saving file:', error)
        }
    }

    function handleKeyDown(event: KeyboardEvent) {
        // Check for Ctrl (Windows/Linux) or Cmd (Mac)
        const isModifier = event.ctrlKey || event.metaKey

        if (isModifier && event.key === 'o') {
            event.preventDefault()
            handleOpenFile()
        } else if (isModifier && event.key === 's') {
            event.preventDefault()
            // Only save if there's data available
            if (dataStore.data) {
                handleSaveAs()
            }
        } else if (event.key === 'F5') {
            event.preventDefault()
            // F5 is handled by SqlEditor if focused.
            // If we're here, it means focus is elsewhere.
            // We prevent default to avoid page reload which clears state.
            if (dataStore.isSqlTabActive) {
                dataStore.triggerQuery()
            }
        }
    }

    function handleContextMenu(event: MouseEvent) {
        // Prevent default browser context menu globally
        event.preventDefault()
    }

    onMount(async () => {
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
            if (isParquetFile(filePath)) {
                loadParquetFile(filePath)
            }
        })

        unlistenFileChanged = await listen<string>('file-changed', async (event) => {
            const filePath = event.payload
            console.log('File changed on disk:', filePath)
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
                    if (isParquetFile(filePath)) {
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
    <div id="toolbar">
        <Toolbar/>
    </div>
    <div id="main-area">{@render children()}</div>
    <div id="footer" class="card">
        <InfoBar/>
        <Footer/>
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
        grid-template-rows: 2.5rem 1fr 3.75rem;
        grid-template-columns: 5rem 1fr;
        grid-template-areas:
      'title-bar title-bar'
      'toolbar main-area'
      'footer footer';
        overflow: hidden;
        background-image: radial-gradient(
                circle at 50% 0,
                var(--surface-3) 0%,
                var(--surface-4) 50%,
                var(--surface-2) 100%
        );
    }

    main.macos,
    main.linux {
        grid-template-rows: 1fr 3.75rem;
        grid-template-areas:
      'toolbar main-area'
      'footer footer';
        padding-top: 0.25rem;
    }


    #title-bar-panel {
        width: 100%;
        grid-area: title-bar;
        display: flex;
        align-items: center;
        justify-content: space-between;
        z-index: 100;
        -webkit-app-region: drag;
        app-region: drag;
        user-select: none;
    }

    #toolbar {
        grid-area: toolbar;
        padding: 0 0.5rem;
    }

    #main-area {
        grid-area: main-area;
        overflow: hidden;
        padding: 0 0.5rem 0 0;
        display: flex;
        flex-direction: column;
    }

    #footer {
        grid-area: footer;
        display: flex;
        flex-direction: column;
        margin-top: 0.25rem;
        background-image: linear-gradient(
                to bottom,
                var(--surface-2),
                var(--surface-3)
        );
        border-top: 1px solid var(--surface-4);
    }
</style>
