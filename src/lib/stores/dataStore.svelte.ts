import {invoke} from '@tauri-apps/api/core'

export interface ColumnInfo {
    name: string
    dtype: string
}

export interface MetadataInfo {
    name: string
    created: string | null
    modified: string | null
    size: number
    row_groups: number
    compression: string
    total_nulls: number
}

export interface ParquetData {
    shape: [number, number]
    columns: ColumnInfo[]
    rows: string[][]
    metadata?: MetadataInfo
}

export interface SortState {
    column: string
    ascending: boolean
}

export interface FileSession {
    id: string
    path: string | null
    name: string
    rawData: ParquetData | null
    queryData: ParquetData | null
    loadingRaw: boolean
    loadingQuery: boolean
    loadingMoreRaw: boolean
    loadingMoreQuery: boolean
    error: string | null
    baseColumns: ColumnInfo[] | null
    columnWidths: Record<number, number>
    sortStates: SortState[]
    tableLayout: 'auto' | 'fixed'
    lastMeasuredColumnsJson: string | null
    dataTag: number // Used to trigger updates without deep proxying rawData/queryData
    isQueryResult: boolean
    isQueryEditor: boolean
    sqlQuery: string | null
    lastSuccessfulQuery: string | null
    originalName: string | null
}

let sessions = $state<FileSession[]>([])
let activeSessionId = $state<string | null>(null)
let isSqlTabActive = $state(false)
let showSqlEditor = $state(false)
let appVersion = $state<string>('0.0.0')
let latestVersion = $state<string | null>(null)
let updateCount = $state<number>(0)
let checkingUpdates = $state<boolean>(false)
let updateCheckError = $state<string | null>(null)
let updateSeen = $state<boolean>(false)
let sqlEditorHeight = $state<number>(0)
const queryListeners = new Set<() => void>()

export const dataStore = {
    usesSqlStorage(session?: FileSession | null, isSqlOverride?: boolean) {
        if (!session) return false
        if (session.isQueryResult) return false
        return isSqlOverride !== undefined ? isSqlOverride : isSqlTabActive
    },
    get sessions() {
        return sessions
    },
    get activeSession() {
        return sessions.find((s) => s.id === activeSessionId) || null
    },
    get activeSessionId() {
        return activeSessionId
    },
    set activeSessionId(id: string | null) {
        activeSessionId = id
        // Don't automatically hide SQL tab when switching sessions
        // Users should be able to switch between files while staying in SQL mode
    },
    get isSqlTabActive() {
        return isSqlTabActive
    },
    set isSqlTabActive(value: boolean) {
        isSqlTabActive = value
    },
    get showSqlEditor() {
        return showSqlEditor
    },
    set showSqlEditor(value: boolean) {
        showSqlEditor = value
    },
    onQueryRequest(cb: () => void) {
        queryListeners.add(cb)
        return () => queryListeners.delete(cb)
    },
    triggerQuery() {
        queryListeners.forEach((cb) => cb())
    },
    get data() {
        const session = this.activeSession
        if (!session) return null

        // Access dataTag and isSqlTabActive to create a dependency on data updates and view switches
        // while keeping the actual data non-reactive to avoid deep proxying
        this.isSqlTabActive
        session.dataTag

        if (session.isQueryResult) {
            return session.rawData
        }

        return this.isSqlTabActive ? session.queryData : session.rawData
    },
    get loading() {
        const session = this.activeSession
        if (!session) return false
        return this.usesSqlStorage(session) ? session.loadingQuery : session.loadingRaw
    },
    get error() {
        return this.activeSession?.error || null
    },
    get loadingMore() {
        const session = this.activeSession
        if (!session) return false
        return this.usesSqlStorage(session)
            ? session.loadingMoreQuery
            : session.loadingMoreRaw
    },
    get hasData() {
        return sessions.length > 0
    },
    get currentQuery() {
        return this.activeSession?.sqlQuery || null
    },
    set currentQuery(value: string | null) {
        const session = this.activeSession
        if (session) {
            session.sqlQuery = value
        }
    },
    get isQueryMode() {
        return (
            this.isSqlTabActive &&
            this.currentQuery !== null &&
            this.currentQuery.trim().length > 0
        )
    },
    get baseColumns() {
        return this.activeSession?.baseColumns || null
    },
    get columnWidths() {
        return this.activeSession?.columnWidths || {}
    },
    set columnWidths(value: Record<number, number>) {
        const session = this.activeSession
        if (session) {
            session.columnWidths = value
        }
    },
    get sortStates() {
        return this.activeSession?.sortStates || []
    },
    set sortStates(value: SortState[]) {
        const session = this.activeSession
        if (session) {
            session.sortStates = value
        }
    },
    get tableLayout() {
        return this.activeSession?.tableLayout || 'auto'
    },
    set tableLayout(value: 'auto' | 'fixed') {
        const session = this.activeSession
        if (session) {
            session.tableLayout = value
        }
    },
    get metadata() {
        const session = this.activeSession
        if (!session) return null
        return (
            this.usesSqlStorage(session)
                ? session.queryData?.metadata
                : session.rawData?.metadata
        ) || null
    },
    get totalRows() {
        const d = this.data
        return d?.shape[0] || 0
    },
    get loadedRows() {
        const d = this.data
        return d?.rows.length || 0
    },
    get appVersion() {
        return appVersion
    },
    get latestVersion() {
        return latestVersion
    },
    get updateCount() {
        return updateCount
    },
    get checkingUpdates() {
        return checkingUpdates
    },
    get updateCheckError() {
        return updateCheckError
    },
    get updateSeen() {
        return updateSeen
    },
    set updateSeen(value: boolean) {
        updateSeen = value
    },
    get sqlEditorHeight() {
        return sqlEditorHeight
    },
    set sqlEditorHeight(value: number) {
        sqlEditorHeight = value
    },

    sanitizeTableName(name: string): string {
        let sanitized = name.replace(/[^A-Za-z0-9_]/g, '_')
        if (/^[0-9]/.test(sanitized)) {
            sanitized = '_' + sanitized
        }
        return sanitized || 'table'
    },

    getTableNames(): Record<string, string> {
        return Object.fromEntries(
            sessions
                .filter((s) => s.path || s.isQueryResult)
                .map((s) => [s.path || s.id, this.sanitizeTableName(s.name)])
        )
    },

    getQueryResults(): { name: string; query: string }[] {
        return sessions
            .filter((s) => s.isQueryResult && s.lastSuccessfulQuery)
            .map((s) => ({
                name: this.sanitizeTableName(s.name),
                query: s.lastSuccessfulQuery!,
            }))
    },

    async checkUpdates() {
        checkingUpdates = true
        updateCheckError = null
        try {
            const {getVersion} = await import('@tauri-apps/api/app')
            appVersion = await getVersion()

            const response = await fetch(
                'https://api.github.com/repos/marepilc/pink-parquet/releases',
                {
                    headers: {
                        'Accept': 'application/vnd.github.v3+json',
                        'User-Agent': 'Pink-Parquet-App'
                    }
                }
            )
            if (!response.ok) {
                const errorMsg = `GitHub releases fetch failed: ${response.statusText}`
                console.warn(errorMsg)
                updateCheckError = errorMsg
                return
            }

            const releases = await response.json()
            if (!Array.isArray(releases) || releases.length === 0) {
                latestVersion = appVersion // Assume up to date if no releases
                updateCount = 0
                return
            }

            latestVersion = releases[0].tag_name.replace(/^v/, '')

            // Simple version comparison for semantic versioning
            function parseVersion(v: string) {
                return v.replace(/^v/, '').split('.').map(Number)
            }

            const current = parseVersion(appVersion)

            let count = 0
            for (const release of releases) {
                const releaseVer = parseVersion(release.tag_name)
                // Compare releaseVer with current
                let isNewer = false
                for (let i = 0; i < Math.max(current.length, releaseVer.length); i++) {
                    const c = current[i] || 0
                    const r = releaseVer[i] || 0
                    if (r > c) {
                        isNewer = true
                        break
                    }
                    if (r < c) {
                        break
                    }
                }
                if (isNewer) {
                    count++
                } else {
                    // Assuming releases are sorted by date/version descending
                    break
                }
            }
            updateCount = count
        } catch (error) {
            const errorMsg = `Failed to check for updates: ${error}`
            console.error(errorMsg)
            updateCheckError = errorMsg
        } finally {
            checkingUpdates = false
        }
    },

    addSession(path: string) {
        const id = crypto.randomUUID()
        const fileName = path.split('\\').pop()?.split('/').pop() || path
        const name = fileName.split('.').slice(0, -1).join('.') || fileName

        const newSession: FileSession = {
            id,
            path,
            name,
            rawData: null,
            queryData: null,
            loadingRaw: false,
            loadingQuery: false,
            loadingMoreRaw: false,
            loadingMoreQuery: false,
            error: null,
            baseColumns: null,
            columnWidths: {},
            sortStates: [],
            tableLayout: 'auto',
            lastMeasuredColumnsJson: null,
            dataTag: 0,
            isQueryResult: false,
            isQueryEditor: false,
            sqlQuery: null,
            lastSuccessfulQuery: null,
            originalName: name,
        }
        sessions.push(newSession)
        activeSessionId = id
        return id
    },

    addQueryEditorSession() {
        const id = crypto.randomUUID()

        // Generate unique name query1, query2, ...
        let index = 1
        let name = `query${index}`
        while (sessions.some((s) => s.name === name)) {
            index++
            name = `query${index}`
        }

        const newSession: FileSession = {
            id,
            path: null,
            name,
            rawData: null,
            queryData: null,
            loadingRaw: false,
            loadingQuery: false,
            loadingMoreRaw: false,
            loadingMoreQuery: false,
            error: null,
            baseColumns: null,
            columnWidths: {},
            sortStates: [],
            tableLayout: 'auto',
            lastMeasuredColumnsJson: null,
            dataTag: 0,
            isQueryResult: false,
            isQueryEditor: true,
            sqlQuery: null,
            lastSuccessfulQuery: null,
            originalName: name,
        }

        sessions.push(newSession)
        activeSessionId = id
        isSqlTabActive = true
        return id
    },

    addQuerySession(query: string, data: ParquetData, sessionId?: string) {
        const id = sessionId || crypto.randomUUID()
        const existingSession = sessions.find((s) => s.id === id)

        if (existingSession) {
            // Keep the editor name if it's already an editor
            if (!existingSession.isQueryEditor) {
                // Generate unique name query1 results, query2 results, ...
                let index = 1
                let name = `query${index} results`
                while (sessions.some((s) => s.name === name)) {
                    index++
                    name = `query${index} results`
                }
                existingSession.name = name
                existingSession.originalName = name
            }

            existingSession.rawData = data
            existingSession.baseColumns = data.columns
            existingSession.sqlQuery = query
            existingSession.lastSuccessfulQuery = query
            existingSession.isQueryResult = true
            existingSession.error = null
            existingSession.dataTag++
            activeSessionId = id
            // Do not hide the SQL editor if it's already an editor
            if (!existingSession.isQueryEditor) {
                isSqlTabActive = false
            }
            return id
        }

        // Generate unique name query1 results, query2 results, ...
        let index = 1
        let name = `query${index} results`
        while (sessions.some((s) => s.name === name)) {
            index++
            name = `query${index} results`
        }

        const newSession: FileSession = {
            id,
            path: null,
            name,
            rawData: data,
            queryData: null,
            loadingRaw: false,
            loadingQuery: false,
            loadingMoreRaw: false,
            loadingMoreQuery: false,
            error: null,
            baseColumns: data.columns,
            columnWidths: {},
            sortStates: [],
            tableLayout: 'auto',
            lastMeasuredColumnsJson: null,
            dataTag: 0,
            isQueryResult: true,
            isQueryEditor: false,
            sqlQuery: query,
            lastSuccessfulQuery: query,
            originalName: name,
        }

        sessions.push(newSession)
        activeSessionId = id
        isSqlTabActive = false
        return id
    },

    async removeSession(id: string) {
        const index = sessions.findIndex((s) => s.id === id)
        if (index !== -1) {
            const path = sessions[index].path
            sessions.splice(index, 1)
            if (activeSessionId === id) {
                activeSessionId =
                    sessions.length > 0 ? sessions[sessions.length - 1].id : null
            }

            // Stop watching the file if no other session is using it
            const otherSessionUsingFile = sessions.some((s) => s.path === path)
            if (path && !otherSessionUsingFile && path.toLowerCase().endsWith('.parquet')) {
                try {
                    await invoke('stop_watching', {filePath: path})
                } catch (e) {
                    console.error('Failed to stop watching file:', e)
                }
            }
        }
    },

    setData(newData: ParquetData, sessionId?: string, isSqlResult?: boolean) {
        const session = sessionId
            ? sessions.find((s) => s.id === sessionId)
            : this.activeSession

        const effectiveIsSql = this.usesSqlStorage(session, isSqlResult)

        if (session) {
            if (effectiveIsSql) {
                session.queryData = newData
                session.loadingQuery = false
                session.error = null
                session.dataTag++
            } else {
                // Raw file data is per-session
                session.rawData = newData
                session.loadingRaw = false
                if (
                    session.sqlQuery === null ||
                    session.sqlQuery.trim().length === 0
                ) {
                    session.baseColumns = newData.columns
                }
                session.error = null
                session.dataTag++
            }
        }
    },

    setQuery(query: string | null) {
        const session = this.activeSession
        if (session) {
            session.sqlQuery = query && query.trim().length > 0 ? query : null
            if (session.sqlQuery === null) {
                session.queryData = null
                session.loadingQuery = false
                session.loadingMoreQuery = false
                session.dataTag++
            }
        }
    },

    resetQueryResults() {
        const session = this.activeSession
        if (session) {
            session.queryData = null
            session.loadingQuery = false
            session.loadingMoreQuery = false
            session.dataTag++
        }
    },

    appendRows(newRows: string[][], sessionId?: string, isSql?: boolean) {
        const session = sessionId
            ? sessions.find((s) => s.id === sessionId)
            : this.activeSession

        const effectiveIsSql = this.usesSqlStorage(session, isSql)

        if (session) {
            if (effectiveIsSql && session.queryData) {
                session.queryData = {
                    ...session.queryData,
                    rows: [...session.queryData.rows, ...newRows],
                }
                session.loadingMoreQuery = false
                session.dataTag++
            } else if (!effectiveIsSql && session.rawData) {
                // Raw file data is per-session
                session.rawData = {
                    ...session.rawData,
                    rows: [...session.rawData.rows, ...newRows],
                }
                session.loadingMoreRaw = false
                session.dataTag++
            }
        }
    },

    setLoading(isLoading: boolean, sessionId?: string, isSql?: boolean) {
        const session = sessionId
            ? sessions.find((s) => s.id === sessionId)
            : this.activeSession
        if (session) {
            const effectiveIsSql = this.usesSqlStorage(session, isSql)
            if (effectiveIsSql) {
                session.loadingQuery = isLoading
            } else {
                // Raw file loading is per-session
                session.loadingRaw = isLoading
            }
        }
    },

    setLoadingMore(isLoading: boolean, sessionId?: string, isSql?: boolean) {
        const session = sessionId
            ? sessions.find((s) => s.id === sessionId)
            : this.activeSession
        if (session) {
            const effectiveIsSql = this.usesSqlStorage(session, isSql)
            if (effectiveIsSql) {
                session.loadingMoreQuery = isLoading
            } else {
                // Raw file loading is per-session
                session.loadingMoreRaw = isLoading
            }
        }
    },

    setError(newError: string, sessionId?: string) {
        const session = sessionId
            ? sessions.find((s) => s.id === sessionId)
            : this.activeSession
        if (session) {
            session.error = newError
            session.loadingRaw = false
            session.loadingQuery = false
            session.loadingMoreRaw = false
            session.loadingMoreQuery = false
            // Clear previous results when error occurs
            if (isSqlTabActive) {
                session.queryData = null
                session.error = newError
                session.loadingQuery = false
                session.loadingMoreQuery = false
                session.dataTag++
            }
        }
    },

    clearError(sessionId?: string) {
        const session = sessionId
            ? sessions.find((s) => s.id === sessionId)
            : this.activeSession
        if (session) {
            session.error = null
        }
    },

    renameSession(id: string, newName: string) {
        const session = sessions.find((s) => s.id === id)
        if (session) {
            session.name = newName
        }
    },

    reorderSession(draggedId: string, targetId: string, position: 'before' | 'after' = 'before') {
        if (!draggedId || draggedId === targetId) return

        const draggedIndex = sessions.findIndex((s) => s.id === draggedId)
        const targetIndex = sessions.findIndex((s) => s.id === targetId)

        if (draggedIndex !== -1 && targetIndex !== -1) {
            const [draggedSession] = sessions.splice(draggedIndex, 1)
            const adjustedTargetIndex =
                draggedIndex < targetIndex ? targetIndex - 1 : targetIndex
            const insertIndex =
                position === 'after' ? adjustedTargetIndex + 1 : adjustedTargetIndex
            sessions.splice(insertIndex, 0, draggedSession)
        }
    },

    async loadParquetFile(
        filePath: string,
        forceReload: boolean = false,
        gotoFn?: (url: string) => Promise<void>
    ) {
        const existingSession = sessions.find((s) => s.path === filePath)
        if (existingSession && !forceReload) {
            activeSessionId = existingSession.id
            if (gotoFn) await gotoFn('/app')
            return
        }

        const sessionId = existingSession
            ? existingSession.id
            : this.addSession(filePath)
        this.setLoading(true, sessionId, false)

        try {
            const data = await invoke('get_data', {
                filePath,
                sorting: null,
            })
            this.setData(data as any, sessionId, false)

            if (gotoFn) await gotoFn('/app')

            // Start watching the file for changes (only for Parquet files)
            if (filePath.toLowerCase().endsWith('.parquet')) {
                await invoke('start_watching', {filePath})
            }
        } catch (error) {
            console.error('Error loading data file:', error)
            this.setError(String(error), sessionId)
        }
    },

    clearData() {
        sessions = []
        activeSessionId = null
    },
}

