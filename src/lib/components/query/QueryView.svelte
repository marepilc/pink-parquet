<script lang="ts">
  import { dataStore } from '$lib/stores/dataStore.svelte'
  import SqlEditor from '$lib/components/SqlEditor.svelte'
  import { invoke } from '@tauri-apps/api/core'

  let { visible = false } = $props<{ visible?: boolean }>()

  let sqlText = $state(dataStore.currentQuery || '')

  $effect(() => {
    sqlText = dataStore.currentQuery || ''
  })

  const keywordCompletions = [
    'join',
    'on',
    'left',
    'right',
    'inner',
    'full',
    'outer',
    'union',
    'all',
    'distinct',
    'select',
    'from',
    'where',
    'like',
    'and',
    'or',
    'order',
    'by',
    'limit',
    'offset',
    'group',
    'having',
    'asc',
    'desc',
    'count',
    'sum',
    'avg',
    'min',
    'max',
    'as',
    'in',
    'is',
    'not',
    'null',
    'between',
    'using',
    'cross',
    'natural',
    'extract',
    'cast',
    'case',
    'when',
    'then',
    'else',
    'end',
    'with',
    'recursive',
  ].map((k) => ({ label: k.toUpperCase(), type: 'keyword' }))

  function sanitizeTableName(name: string): string {
    let sanitized = name.replace(/[^A-Za-z0-9_]/g, '_')
    if (/^[0-9]/.test(sanitized)) {
      sanitized = '_' + sanitized
    }
    return sanitized || 'table'
  }

  function getTableAndColumnCompletions(sql: string) {
    const tableCompletions: { label: string; type: string }[] = []
    const columnNamesSet = new Set<string>()
    const tableToColumns: Record<string, string[]> = {}
    const aliasToTable: Record<string, string> = {}

    dataStore.sessions.forEach((session) => {
      const name = sanitizeTableName(session.name)
      tableCompletions.push({ label: name, type: 'class' })

      const cols = (session.baseColumns || session.rawData?.columns || []).map(
        (c) => c.name
      )
      tableToColumns[name] = cols
      cols.forEach((c) => {
        columnNamesSet.add(c)
      })
    })

    // Very basic alias extraction: FROM table AS alias or FROM table alias
    // Matches: FROM/JOIN <tableName> (AS)? <alias>
    const aliasRegex =
      /\b(?:FROM|JOIN)\s+([A-Za-z0-9_]+)\s+(?:AS\s+)?([A-Za-z0-9_]+)/gi
    let match
    while ((match = aliasRegex.exec(sql)) !== null) {
      const tableName = match[1]
      const alias = match[2]

      // Skip if alias is a keyword
      if (
        [
          'WHERE',
          'GROUP',
          'ORDER',
          'LIMIT',
          'OFFSET',
          'JOIN',
          'LEFT',
          'RIGHT',
          'INNER',
          'FULL',
          'ON',
        ].includes(alias.toUpperCase())
      ) {
        continue
      }

      // Only if tableName is one of our known tables
      if (tableToColumns[tableName]) {
        aliasToTable[alias] = tableName
      }
    }

    const columnCompletions = Array.from(columnNamesSet).map((name) => ({
      label: name,
      type: 'variable',
    }))

    return { tableCompletions, columnCompletions, tableToColumns, aliasToTable }
  }

  const completionsData = $derived(getTableAndColumnCompletions(sqlText))
  const { tableCompletions, columnCompletions, tableToColumns, aliasToTable } =
    $derived(completionsData)

  function getAllCompletions() {
    return (keywordCompletions as any).concat(
      tableCompletions,
      columnCompletions
    )
  }

  async function runSql() {
    if (!dataStore.activeSession?.path) return
    const q = sqlText.trim()
    if (!q) return

    dataStore.setQuery(q)
    dataStore.setLoading(true, undefined, true)

    try {
      // Ensure we are in SQL tab mode
      dataStore.isSqlTabActive = true

      const tableNames = Object.fromEntries(
        dataStore.sessions.map((s) => [s.path, sanitizeTableName(s.name)])
      )

      const newData = await invoke('execute_sql', {
        activeFilePath: dataStore.activeSession?.path,
        allFiles: dataStore.sessions.map((s) => s.path),
        tableNames,
        query: q,
        offset: 0,
        limit: 100,
      })

      dataStore.setData(newData as any, undefined, true)
    } catch (error) {
      console.error('QueryView: SQL execution failed', error)
      dataStore.setError(String(error))
    }
  }

  async function clearSql() {
    sqlText = ''
    dataStore.setQuery(null)
    // When clearing SQL, we just want to clear the queryData in the store,
    // not fetch raw data again into the rawData slot which might be confusing.
    const session = dataStore.activeSession
    if (session) {
      session.queryData = null
    }
  }
</script>

<div class="query-container">
  <div class="query-content">
    <div class="editor-wrapper">
      <SqlEditor
        bind:value={sqlText}
        onChange={(v) => {
          dataStore.setQuery(v)
        }}
        placeholder="Enter SQL query here"
        tableName={tableCompletions.map((t) => t.label)}
        columns={columnCompletions}
        {tableToColumns}
        {aliasToTable}
        onRun={runSql}
        completions={getAllCompletions()}
        {visible}
      />
    </div>
    <div class="button-group">
      <button class="run-button" title="Run SQL (F5)" onclick={runSql}
        >Run
      </button>
      <button class="clear-button" title="Clear SQL" onclick={clearSql}
        >Clear
      </button>
    </div>
  </div>
</div>

<style>
  .query-container {
    flex-shrink: 0;
    background: var(--surface-2);
    border-bottom: 1px solid var(--surface-4);
    border-bottom: 1px solid oklch(from var(--surface-4) l c h / 0.6);
    padding: 0.75rem;
  }

  .query-content {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
  }

  .editor-wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .button-group {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    gap: 0.5rem;
  }

  .run-button {
    padding: 0.5rem 1rem;
    font-size: var(--text-sm);
    font-weight: 600;
    color: white;
    background: var(--accent);
    border: 1px solid var(--accent);
    border: 1px solid oklch(from var(--accent) calc(l * 0.8) c h);
    border-radius: 0.25rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .run-button:hover {
    filter: brightness(1.1);
    background: oklch(from var(--accent) calc(l * 0.9) c h);
  }

  .run-button:active {
    transform: scale(0.98);
  }

  .clear-button {
    padding: 0.25rem 0.5rem;
    font-size: var(--text-xs);
    color: var(--ink-3);
    background: transparent;
    border: 1px solid var(--surface-5);
    border-radius: 0.25rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .clear-button:hover {
    color: var(--ink-4);
    background: var(--surface-3);
    border-color: var(--surface-6);
  }
</style>
