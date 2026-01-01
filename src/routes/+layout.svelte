<script lang="ts">
    import '../app.css'
    import TitleBar from '$lib/components/TitleBar.svelte'
    import Toolbar from '$lib/components/Toolbar.svelte'
    import Footer from '$lib/components/Footer.svelte'
    import DragOverlay from '$lib/components/DragOverlay.svelte'
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

    interface Props {
        children: Snippet
    }

    let {children}: Props = $props()

    let unlistenDrag: UnlistenFn | null = null
    let unlistenOpenFile: UnlistenFn | null = null
    let isDraggingValidFile = $state(false)

    function isValidFileType(filePath: string): boolean {
        return filePath.endsWith('.parquet') || filePath.endsWith('.sql')
    }

    function isParquetFile(filePath: string): boolean {
        return filePath.endsWith('.parquet')
    }

    async function loadParquetFile(filePath: string) {
        // Check if a file is already open
        const existingSession = dataStore.sessions.find((s) => s.path === filePath)
        if (existingSession) {
            // File already open, just switch to it
            dataStore.activeSessionId = existingSession.id
            // Navigate to /app if not already there
            await goto('/app')
            return
        }

        const sessionId = dataStore.addSession(filePath)
        dataStore.setLoading(true, sessionId)

        try {
            const data = await invoke('get_data', {
                filePath,
                sorting: null,
            })
            dataStore.setData(data as any, sessionId, false)

            // Navigate to /app after a successful load
            await goto('/app')
        } catch (error) {
            console.error('Error loading Parquet file:', error)
            dataStore.setError(String(error), sessionId)
        }
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
        }
    }

    onMount(async () => {
        // Initialize settings
        await settingsStore.init()

        unlistenOpenFile = await listen<string>('open-file', (event) => {
            const filePath = event.payload
            if (isParquetFile(filePath)) {
                loadParquetFile(filePath)
            }
        })

        unlistenDrag = await getCurrentWebview().onDragDropEvent((event) => {
            const payload = event.payload as { type: string; paths?: string[] }

            if (payload.type === 'enter' || payload.type === 'over') {
                const paths = payload.paths
                if (paths && paths.length > 0) {
                    const filePath = paths[0]
                    if (isValidFileType(filePath)) {
                        isDraggingValidFile = true
                    }
                }
            } else if (payload.type === 'drop') {
                isDraggingValidFile = false
                const paths = payload.paths

                if (paths && paths.length > 0) {
                    const filePath = paths[0]
                    if (isParquetFile(filePath)) {
                        loadParquetFile(filePath)
                    }
                    // SQL files: do nothing for now (future implementation)
                }
            } else if (payload.type === 'leave' || payload.type === 'cancel') {
                isDraggingValidFile = false
            }
        })
    })

    onDestroy(() => {
        if (unlistenDrag) unlistenDrag()
        if (unlistenOpenFile) unlistenOpenFile()
    })
</script>

<svelte:window onkeydown={handleKeyDown}/>

<main class="main-window">
    <header id="title-bar-panel" data-tauri-drag-region>
        <TitleBar/>
    </header>
    <div id="toolbar">
        <Toolbar/>
    </div>
    <div id="main-area">{@render children()}</div>
    <div id="footer" class="card">
        <InfoBar/>
        <Footer/>
    </div>

    <DragOverlay isVisible={isDraggingValidFile} hasData={dataStore.hasData}/>
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
