import {invoke} from '@tauri-apps/api/core'
import {open, save} from '@tauri-apps/plugin-dialog'
import {goto} from '$app/navigation'
import {dataStore} from '$lib/stores/dataStore.svelte'

export async function loadDataFile(filePath: string, forceReload: boolean = false) {
    await dataStore.loadParquetFile(filePath, forceReload, goto)
}

export async function openDataFiles() {
    try {
        const files = await open({
            multiple: true,
            filters: [
                {
                    name: 'Data Files',
                    extensions: ['parquet', 'csv'],
                },
            ],
        })

        if (files && Array.isArray(files)) {
            for (const file of files) {
                await loadDataFile(file)
            }
        } else if (files) {
            await loadDataFile(files as string)
        }
    } catch (error) {
        console.error('Error opening file:', error)
    }
}

// Suggests "<original dir>/<session name>.parquet", or untitled.parquet for SQL results
function defaultSavePath(): string | undefined {
    if (dataStore.isSqlTabActive) {
        return 'untitled.parquet'
    }

    const activeSession = dataStore.sessions.find((s) => s.id === dataStore.activeSessionId)
    if (!activeSession) return undefined

    let name = activeSession.name
    if (name.toLowerCase().endsWith('.csv')) {
        name = name.substring(0, name.length - 4)
    }
    if (!name.toLowerCase().endsWith('.parquet')) {
        name = `${name}.parquet`
    }

    const originalPath = activeSession.path ?? ''
    const pathSeparator = originalPath.includes('/') ? '/' : '\\'
    const lastSeparatorIndex = originalPath.lastIndexOf(pathSeparator)
    if (lastSeparatorIndex < 0) return name

    return originalPath.substring(0, lastSeparatorIndex + 1) + name
}

export async function saveActiveAs() {
    if (!dataStore.data) return

    try {
        const filePath = await save({
            defaultPath: defaultSavePath(),
            filters: [
                {
                    name: 'Parquet',
                    extensions: ['parquet'],
                },
                {
                    name: 'CSV',
                    extensions: ['csv'],
                },
            ],
        })

        if (filePath) {
            const command = filePath.toLowerCase().endsWith('.csv') ? 'save_csv' : 'save_parquet'
            await invoke(command, {filePath})

            // If the saved file is already open in any session, reload that session
            const existingSession = dataStore.sessions.find((s) => s.path === filePath)
            if (existingSession) {
                await loadDataFile(filePath, true)
            }
        }
    } catch (error) {
        console.error('Error saving file:', error)
    }
}
