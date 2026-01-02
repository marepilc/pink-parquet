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

export interface FileSession {
    id: string
    path: string
    name: string
    rawData: ParquetData | null
    queryData: ParquetData | null
    loadingRaw: boolean
    loadingQuery: boolean
    loadingMoreRaw: boolean
    loadingMoreQuery: boolean
    error: string | null
    currentQuery: string | null
    baseColumns: ColumnInfo[] | null
    columnWidths: Record<number, number>
    tableLayout: 'auto' | 'fixed'
    lastMeasuredColumnsJson: string | null
    dataTag: number // Used to trigger updates without deep proxying rawData/queryData
}

let sessions = $state<FileSession[]>([])
let activeSessionId = $state<string | null>(null)
let isSqlTabActive = $state(false)
const queryListeners = new Set<() => void>()

export const dataStore = {
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

        return this.isSqlTabActive ? session.queryData : session.rawData
    },
    get loading() {
        const session = this.activeSession
        if (!session) return false
        return isSqlTabActive ? session.loadingQuery : session.loadingRaw
    },
    get error() {
        return this.activeSession?.error || null
    },
    get loadingMore() {
        const session = this.activeSession
        if (!session) return false
        return isSqlTabActive ? session.loadingMoreQuery : session.loadingMoreRaw
    },
    get hasData() {
        return sessions.length > 0
    },
    get currentQuery() {
        return this.activeSession?.currentQuery || null
    },
    set currentQuery(value: string | null) {
        this.setQuery(value)
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
        return this.activeSession?.rawData?.metadata || null
    },
    get totalRows() {
        const d = this.data
        return d?.shape[0] || 0
    },
    get loadedRows() {
        const d = this.data
        return d?.rows.length || 0
    },

    addSession(path: string) {
        const id = crypto.randomUUID()
        const fileName = path.split('\\').pop()?.split('/').pop() || path
        const name = fileName.split('.').slice(0, -1).join('.') || fileName

        // Inherit query from any existing session (they all have the same query now)
        const sharedQuery = sessions.length > 0 ? sessions[0].currentQuery : null

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
            currentQuery: sharedQuery,
            baseColumns: null,
            columnWidths: {},
            tableLayout: 'auto',
            lastMeasuredColumnsJson: null,
            dataTag: 0,
        }
        sessions.push(newSession)
        activeSessionId = id
        return id
    },

    removeSession(id: string) {
        const index = sessions.findIndex((s) => s.id === id)
        if (index !== -1) {
            sessions.splice(index, 1)
            if (activeSessionId === id) {
                activeSessionId =
                    sessions.length > 0 ? sessions[sessions.length - 1].id : null
            }
        }
    },

    setData(newData: ParquetData, sessionId?: string, isSqlResult?: boolean) {
        const session = sessionId
            ? sessions.find((s) => s.id === sessionId)
            : this.activeSession

        const effectiveIsSql =
            isSqlResult !== undefined ? isSqlResult : isSqlTabActive

        if (session) {
            if (effectiveIsSql) {
                // SQL results are shared across all sessions (global SQL state)
                sessions.forEach((s) => {
                    s.queryData = newData
                    s.loadingQuery = false
                    s.error = null
                    s.dataTag++
                })
            } else {
                // Raw file data is per-session
                session.rawData = newData
                session.loadingRaw = false
                if (
                    session.currentQuery === null ||
                    session.currentQuery.trim().length === 0
                ) {
                    session.baseColumns = newData.columns
                }
                session.error = null
                session.dataTag++
            }
        }
    },

    setQuery(query: string | null) {
        const normalizedQuery = query && query.trim().length > 0 ? query : null

        // Update all sessions with the same query to keep them in sync
        // This ensures the query is visible regardless of which session is active
        sessions.forEach((session) => {
            session.currentQuery = normalizedQuery
            if (normalizedQuery === null) {
                session.queryData = null
                session.loadingQuery = false
                session.loadingMoreQuery = false
                session.dataTag++
            }
        })
    },

    resetQueryResults() {
        sessions.forEach((session) => {
            session.queryData = null
            session.loadingQuery = false
            session.loadingMoreQuery = false
            session.dataTag++
        })
    },

    appendRows(newRows: string[][], sessionId?: string, isSql?: boolean) {
        const session = sessionId
            ? sessions.find((s) => s.id === sessionId)
            : this.activeSession

        const effectiveIsSql = isSql !== undefined ? isSql : this.isSqlTabActive

        if (session) {
            if (effectiveIsSql && session.queryData) {
                // SQL results are shared across all sessions
                const updatedQueryData = {
                    ...session.queryData,
                    rows: [...session.queryData.rows, ...newRows],
                }
                sessions.forEach((s) => {
                    s.queryData = updatedQueryData
                    s.loadingMoreQuery = false
                    s.dataTag++
                })
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
            const effectiveIsSql = isSql !== undefined ? isSql : this.isSqlTabActive
            if (effectiveIsSql) {
                // SQL loading state is shared across all sessions
                sessions.forEach((s) => {
                    s.loadingQuery = isLoading
                })
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
            const effectiveIsSql = isSql !== undefined ? isSql : this.isSqlTabActive
            if (effectiveIsSql) {
                // SQL loading state is shared across all sessions
                sessions.forEach((s) => {
                    s.loadingMoreQuery = isLoading
                })
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
                sessions.forEach((s) => {
                    s.queryData = null
                    s.error = newError
                    s.loadingQuery = false
                    s.loadingMoreQuery = false
                    s.dataTag++
                })
            }
        }
    },

    clearError(sessionId?: string) {
        const session = sessionId
            ? sessions.find((s) => s.id === sessionId)
            : this.activeSession
        if (session) {
            if (isSqlTabActive) {
                sessions.forEach((s) => {
                    s.error = null
                })
            } else {
                session.error = null
            }
        }
    },

    renameSession(id: string, newName: string) {
        const session = sessions.find((s) => s.id === id)
        if (session) {
            session.name = newName
        }
    },

    clearData() {
        sessions = []
        activeSessionId = null
    },
}
