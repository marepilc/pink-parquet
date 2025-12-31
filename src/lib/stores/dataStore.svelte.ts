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
    if (id !== null) {
      isSqlTabActive = false
    }
  },
  get isSqlTabActive() {
    return isSqlTabActive
  },
  set isSqlTabActive(value: boolean) {
    isSqlTabActive = value
  },
  get data() {
    const session = sessions.find((s) => s.id === activeSessionId)
    if (!session) {
      return null
    }

    // Access dataTag and isSqlTabActive to create a dependency on data updates and view switches
    // while keeping the actual data non-reactive to avoid deep proxying
    const isSql = this.isSqlTabActive

    return isSql ? session.queryData : session.rawData
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

    // Inherit query from currently active session if it exists
    const previousQuery = this.activeSession?.currentQuery || null

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
      currentQuery: previousQuery,
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
        session.queryData = newData
        session.loadingQuery = false
      } else {
        session.rawData = newData
        session.loadingRaw = false
        if (
          session.currentQuery === null ||
          session.currentQuery.trim().length === 0
        ) {
          session.baseColumns = newData.columns
        }
      }
      session.error = null
      session.dataTag++
    }
  },

  setQuery(query: string | null, sessionId?: string) {
    const session = sessionId
      ? sessions.find((s) => s.id === sessionId)
      : this.activeSession
    if (session) {
      session.currentQuery = query && query.trim().length > 0 ? query : null
      if (session.currentQuery === null) {
        session.queryData = null
        session.loadingQuery = false
        session.loadingMoreQuery = false
        session.dataTag++
      }
    }
  },

  appendRows(newRows: string[][], sessionId?: string, isSql?: boolean) {
    const session = sessionId
      ? sessions.find((s) => s.id === sessionId)
      : this.activeSession

    const effectiveIsSql = isSql !== undefined ? isSql : this.isSqlTabActive

    if (session) {
      if (effectiveIsSql && session.queryData) {
        session.queryData = {
          ...session.queryData,
          rows: [...session.queryData.rows, ...newRows],
        }
        session.loadingMoreQuery = false
        session.dataTag++
      } else if (!effectiveIsSql && session.rawData) {
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
        session.loadingQuery = isLoading
      } else {
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
        session.loadingMoreQuery = isLoading
      } else {
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

  clearData() {
    sessions = []
    activeSessionId = null
  },
}
