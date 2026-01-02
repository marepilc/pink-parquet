<script lang="ts">
  import { dataStore } from '$lib/stores/dataStore.svelte'
  import { tooltipStore } from '$lib/stores/tooltipStore.svelte'
  import { invoke } from '@tauri-apps/api/core'
  import { onMount, untrack } from 'svelte'
  import QueryGuide from '$lib/components/query/QueryGuide.svelte'

  // Externalized icons as Svelte components
  import DtypeIntIcon from '$lib/components/icons/DtypeInt.svelte'
  import DtypeFloatIcon from '$lib/components/icons/DtypeFloat.svelte'
  import DtypeStringIcon from '$lib/components/icons/DtypeString.svelte'
  import DtypeBoolIcon from '$lib/components/icons/DtypeBool.svelte'
  import DtypeDateIcon from '$lib/components/icons/DtypeDate.svelte'
  import DtypeBinaryIcon from '$lib/components/icons/DtypeBinary.svelte'
  import DtypeNullIcon from '$lib/components/icons/DtypeNull.svelte'
  import DtypeCategoricalIcon from '$lib/components/icons/DtypeCategorical.svelte'
  import DtypeDatetimeIcon from '$lib/components/icons/DtypeDatetime.svelte'
  import DtypeDurationIcon from '$lib/components/icons/DtypeDuration.svelte'
  import DtypeEnumIcon from '$lib/components/icons/DtypeEnum.svelte'
  import DtypeListIcon from '$lib/components/icons/DtypeList.svelte'
  import DtypeArrayIcon from '$lib/components/icons/DtypeArray.svelte'
  import DtypeStructIcon from '$lib/components/icons/DtypeStruct.svelte'
  import DtypeTimeIcon from '$lib/components/icons/DtypeTime.svelte'
  import DtypeCalendarIcon from '$lib/components/icons/DtypeCalendar.svelte'

  function dtypeIcon(dtype: string) {
    const lc = (dtype || '').toLowerCase()
    if (lc.includes('categorical')) return DtypeCategoricalIcon
    if (lc.includes('datetime')) return DtypeDatetimeIcon
    if (lc.includes('duration')) return DtypeDurationIcon
    if (lc.includes('enum')) return DtypeEnumIcon
    if (lc.includes('array')) return DtypeArrayIcon
    if (lc.includes('list')) return DtypeListIcon
    if (lc.includes('struct')) return DtypeStructIcon
    if (lc.includes('time') && !lc.includes('datetime')) return DtypeTimeIcon
    if (lc.includes('calendar')) return DtypeCalendarIcon
    if (lc.includes('int')) return DtypeIntIcon
    if (lc.includes('float') || lc.includes('double') || lc.includes('decimal'))
      return DtypeFloatIcon
    if (lc.includes('bool')) return DtypeBoolIcon
    if (lc.includes('date')) return DtypeDateIcon
    if (lc.includes('str') || lc.includes('char') || lc.includes('utf'))
      return DtypeStringIcon
    if (lc.includes('bin')) return DtypeBinaryIcon
    if (lc.includes('null')) return DtypeNullIcon
    return DtypeStringIcon
  }

  let tableContainer: HTMLDivElement | undefined = $state()
  const BATCH_SIZE = 100
  const MIN_COL_WIDTH = 100
  const MAX_COL_WIDTH_PERCENT = 0.8 // Max 80% of window width

  const data = $derived(dataStore.data)
  const columns = $derived(data?.columns || [])
  const rows = $derived(data?.rows || [])

  const totalRows = $derived(dataStore.totalRows)
  const loadedRows = $derived(dataStore.loadedRows)
  const hasMoreRows = $derived(loadedRows < totalRows)
  const isLoadingMore = $derived(dataStore.loadingMore)

  // Column sizing & resizing (persisted in dataStore)
  let resizingColumn = $state<number | null>(null)
  let resizeStartX = $state(0)
  let resizeStartWidth = $state(0)

  const columnWidths = $derived(dataStore.columnWidths)
  const tableLayout = $derived(dataStore.tableLayout)

  // Track header cells to measure natural widths
  let headerCells: HTMLTableCellElement[] = $state([])

  // Column sorting - multiple columns support
  interface SortState {
    column: string
    ascending: boolean
  }
  let sortStates = $state<SortState[]>([])

  // Context menu
  let contextMenu = $state<{
    visible: boolean
    x: number
    y: number
    type: 'cell' | 'header'
    rowIndex?: number
    colIndex?: number
  }>({ visible: false, x: 0, y: 0, type: 'cell' })

  let contextMenuElement: HTMLElement | undefined = $state()

  // Column stats popover
  let statsPopover = $state<{
    columnIndex: number | null
    stats: any
    histogram: {
      bins: number[]
      counts: number[]
      min: number
      max: number
    } | null
    loading: boolean
    isOpen: boolean
  }>({
    columnIndex: null,
    stats: null,
    histogram: null,
    loading: false,
    isOpen: false,
  })

  let statsPopoverElement: HTMLElement | undefined = $state()

  // Histogram tooltip
  let histogramTooltip = $state<{
    text: string
    x: number
    y: number
  }>({ text: '', x: 0, y: 0 })

  let histogramTooltipElement: HTMLElement | undefined = $state()

  function formatNumber(value: number, decimals: number = 2): string {
    const fixed = value.toFixed(decimals)
    // Only remove trailing zeros if there's a decimal point
    if (!fixed.includes('.')) {
      return fixed // Integer, no decimal point - keep as-is
    }
    // Remove trailing zeros after decimal point
    return fixed.replace(/\.?0+$/, '')
  }

  function showHistogramTooltip(
    event: MouseEvent,
    binIndex: number,
    count: number,
    bins: number[]
  ) {
    const rect = (event.target as SVGElement).getBoundingClientRect()
    const binStart = bins[binIndex]
    const binEnd = bins[binIndex + 1]
    const rangeText = `${formatNumber(binStart, 2)} - ${formatNumber(binEnd, 2)}`
    const countText = `Count: ${formatNumber(count, 0)}`

    histogramTooltip = {
      text: `${rangeText}\n${countText}`,
      x: rect.left + rect.width / 2,
      y: rect.top - 8,
    }

    if (histogramTooltipElement) {
      histogramTooltipElement.style.left = `${histogramTooltip.x}px`
      histogramTooltipElement.style.top = `${histogramTooltip.y}px`
      histogramTooltipElement.showPopover()
    }
  }

  function hideHistogramTooltip() {
    if (histogramTooltipElement) {
      histogramTooltipElement.hidePopover()
    }
  }

  function startResize(event: MouseEvent, columnIndex: number) {
    event.preventDefault()
    const th = (event.target as HTMLElement).closest('th')
    if (!th) return

    resizingColumn = columnIndex
    resizeStartX = event.clientX
    resizeStartWidth =
      dataStore.columnWidths[columnIndex] || th.offsetWidth || MIN_COL_WIDTH

    document.addEventListener('mousemove', handleResize)
    document.addEventListener('mouseup', stopResize)
  }

  function handleResize(event: MouseEvent) {
    if (resizingColumn === null) return

    let diff = event.clientX - resizeStartX
    const maxWidth = window.innerWidth * MAX_COL_WIDTH_PERCENT

    const newWidths = { ...dataStore.columnWidths }
    newWidths[resizingColumn] = Math.max(
      MIN_COL_WIDTH,
      Math.min(maxWidth, resizeStartWidth + diff)
    )
    dataStore.columnWidths = newWidths
  }

  function stopResize() {
    resizingColumn = null
    document.removeEventListener('mousemove', handleResize)
    document.removeEventListener('mouseup', stopResize)
  }

  async function handleSortClick(columnName: string, ascending: boolean) {
    if (!dataStore.activeSession?.path) return

    const existingIndex = sortStates.findIndex((s) => s.column === columnName)

    if (existingIndex !== -1) {
      const currentSort = sortStates[existingIndex]

      if (currentSort.ascending === ascending) {
        // Clicking the same direction toggles off (remove this column from multi-sort)
        sortStates = sortStates.filter((s) => s.column !== columnName)
      } else {
        // Change direction for this column, keep its position in the ordering
        sortStates[existingIndex] = { column: columnName, ascending }
        sortStates = [...sortStates]
      }
    } else {
      // Add a new column to the end for multi-sorting
      sortStates = [...sortStates, { column: columnName, ascending }]
    }

    // Reload data with a new sort
    await reloadData()
  }

  function getSortOrder(columnName: string): number | null {
    const index = sortStates.findIndex((s) => s.column === columnName)
    return index !== -1 ? index + 1 : null
  }

  function getSortDirection(columnName: string): boolean | null {
    const sort = sortStates.find((s) => s.column === columnName)
    return sort ? sort.ascending : null
  }

  function sanitizeTableName(name: string): string {
    let sanitized = name.replace(/[^A-Za-z0-9_]/g, '_')
    if (/^[0-9]/.test(sanitized)) {
      sanitized = '_' + sanitized
    }
    return sanitized || 'table'
  }

  async function reloadData() {
    if (!dataStore.activeSession?.path) return
    const isQuery = dataStore.isSqlTabActive && dataStore.isQueryMode
    dataStore.setLoading(true, undefined, isQuery)

    try {
      const sorting = !isQuery && sortStates.length > 0 ? sortStates : null

      let newData: any
      if (isQuery && dataStore.currentQuery) {
        const tableNames = Object.fromEntries(
          dataStore.sessions.map((s) => [s.path, sanitizeTableName(s.name)])
        )

        newData = await invoke('execute_sql', {
          activeFilePath: dataStore.activeSession?.path,
          allFiles: dataStore.sessions.map((s) => s.path),
          tableNames,
          query: dataStore.currentQuery,
          offset: 0,
          limit: BATCH_SIZE,
        })
      } else {
        newData = await invoke('get_data', {
          filePath: dataStore.activeSession?.path,
          sorting,
        })
      }

      dataStore.setData(newData as any, undefined, isQuery)
    } catch (error) {
      console.error('Error reloading data:', error)
      dataStore.setError(String(error))
    }
  }

  async function loadMoreRows() {
    if (isLoadingMore || !hasMoreRows || !dataStore.activeSession?.path) return

    const isQuery = dataStore.isSqlTabActive && dataStore.isQueryMode
    dataStore.setLoadingMore(true, undefined, isQuery)

    try {
      const sorting = !isQuery && sortStates.length > 0 ? sortStates : null

      let newRows: string[][]
      if (isQuery && dataStore.currentQuery) {
        const tableNames = Object.fromEntries(
          dataStore.sessions.map((s) => [s.path, sanitizeTableName(s.name)])
        )

        newRows = await invoke<string[][]>('get_more_sql_rows', {
          allFiles: dataStore.sessions.map((s) => s.path),
          tableNames,
          query: dataStore.currentQuery,
          offset: loadedRows,
          limit: BATCH_SIZE,
        })
      } else {
        newRows = await invoke<string[][]>('get_more_rows', {
          filePath: dataStore.activeSession?.path,
          offset: loadedRows,
          limit: BATCH_SIZE,
          sorting,
        })
      }

      dataStore.appendRows(newRows, undefined, isQuery)
    } catch (error) {
      console.error('Error loading more rows:', error)
      dataStore.setLoadingMore(false, undefined, isQuery)
    }
  }

  function handleCellContextMenu(
    event: MouseEvent,
    rowIndex: number,
    colIndex: number
  ) {
    event.preventDefault()
    tooltipStore.hideImmediate()

    // Set initial position
    contextMenu = {
      visible: true,
      x: event.clientX,
      y: event.clientY,
      type: 'cell',
      rowIndex,
      colIndex,
    }

    // Adjust position to stay within viewport
    adjustContextMenuPosition()
  }

  function handleHeaderContextMenu(event: MouseEvent, colIndex: number) {
    event.preventDefault()
    tooltipStore.hideImmediate()

    // Set initial position
    contextMenu = {
      visible: true,
      x: event.clientX,
      y: event.clientY,
      type: 'header',
      colIndex,
    }

    // Adjust position to stay within viewport
    adjustContextMenuPosition()
  }

  function adjustContextMenuPosition() {
    if (!contextMenuElement) {
      // Menu not rendered yet, defer adjustment
      queueMicrotask(() => adjustContextMenuPosition())
      return
    }

    const rect = contextMenuElement.getBoundingClientRect()
    const viewportWidth = window.innerWidth
    const viewportHeight = window.innerHeight
    const padding = 16

    let { x, y } = contextMenu

    // Adjust horizontal position if overflowing right edge
    if (x + rect.width > viewportWidth - padding) {
      x = viewportWidth - rect.width - padding
    }

    // Ensure not too far left
    if (x < padding) {
      x = padding
    }

    // Adjust vertical position if overflowing bottom edge
    if (y + rect.height > viewportHeight - padding) {
      y = viewportHeight - rect.height - padding
    }

    // Ensure not too far up
    if (y < padding) {
      y = padding
    }

    // Update position if it changed
    if (x !== contextMenu.x || y !== contextMenu.y) {
      contextMenu = { ...contextMenu, x, y }
    }
  }

  function closeContextMenu() {
    contextMenu = { visible: false, x: 0, y: 0, type: 'cell' }
  }

  async function copyValue() {
    if (
      contextMenu.rowIndex !== undefined &&
      contextMenu.colIndex !== undefined
    ) {
      const value = rows[contextMenu.rowIndex][contextMenu.colIndex]
      await navigator.clipboard.writeText(value)
    }
    closeContextMenu()
  }

  async function copyRow() {
    if (contextMenu.rowIndex !== undefined) {
      const rowData = rows[contextMenu.rowIndex].join('\t')
      await navigator.clipboard.writeText(rowData)
    }
    closeContextMenu()
  }

  async function copyTable() {
    try {
      // Use Rust backend to get the full table (not just loaded rows)
      const tableData = await invoke<string>('copy_full_table')
      await navigator.clipboard.writeText(tableData)
    } catch (error) {
      console.error('Error copying table:', error)
      // Fallback to copying loaded rows if a backend fails
      const tableData = rows.map((row) => row.join('\t')).join('\n')
      await navigator.clipboard.writeText(tableData)
    }
    closeContextMenu()
  }

  async function copyColumnName() {
    if (contextMenu.colIndex !== undefined) {
      const columnName = columns[contextMenu.colIndex]?.name || ''
      await navigator.clipboard.writeText(columnName)
    }
    closeContextMenu()
  }

  async function copyHeaderRow() {
    const headerRow = columns.map((col) => col.name).join('\t')
    await navigator.clipboard.writeText(headerRow)
    closeContextMenu()
  }

  async function handleHeaderClick(event: MouseEvent, colIndex: number) {
    // Don't trigger if clicking on sort buttons or resize handle
    if ((event.target as HTMLElement).closest('.sort-button, .resize-handle')) {
      return
    }

    // Toggle: if clicking the same column that's already showing stats, close it
    if (statsPopover.isOpen && statsPopover.columnIndex === colIndex) {
      if (statsPopoverElement) {
        statsPopoverElement.hidePopover()
      }
      statsPopover.isOpen = false
      return
    }

    const headerCell = event.currentTarget as HTMLElement

    statsPopover = {
      columnIndex: colIndex,
      stats: null,
      histogram: null,
      loading: true,
      isOpen: true,
    }

    // Show the popover
    if (statsPopoverElement) {
      statsPopoverElement.showPopover()

      // Position the popover below the header cell, aligned to a viewport
      const rect = headerCell.getBoundingClientRect()
      const popoverRect = statsPopoverElement.getBoundingClientRect()

      // Position below the header
      let top = rect.bottom + 8
      let left = rect.left

      // Ensure popover stays within viewport horizontally
      const viewportWidth = window.innerWidth
      if (left + popoverRect.width > viewportWidth - 16) {
        left = viewportWidth - popoverRect.width - 16
      }
      if (left < 16) {
        left = 16
      }

      // Ensure popover stays within viewport vertically
      const viewportHeight = window.innerHeight
      if (top + popoverRect.height > viewportHeight - 16) {
        // Position above the header instead
        top = rect.top - popoverRect.height - 8
      }

      statsPopoverElement.style.left = `${left}px`
      statsPopoverElement.style.top = `${top}px`
    }

    // Fetch statistics for this column
    try {
      if (!dataStore.activeSession?.path) return

      const columnName = columns[colIndex]?.name
      if (!columnName) return

      const isQuery = dataStore.isSqlTabActive && dataStore.isQueryMode

      // Fetch basic statistics
      let allStats: Record<string, Record<string, any>>

      if (isQuery && dataStore.currentQuery) {
        // For SQL queries, use the cached query result
        allStats = await invoke<Record<string, Record<string, any>>>(
          'get_query_statistics',
          {
            query: dataStore.currentQuery,
          }
        )
      } else {
        // For regular files, load from file path
        allStats = await invoke<Record<string, Record<string, any>>>(
          'get_statistics',
          {
            filePath: dataStore.activeSession.path,
          }
        )
      }

      if (allStats[columnName]) {
        statsPopover.stats = allStats[columnName]
      }

      // Fetch histogram for numeric columns
      if (isNumericColumn(colIndex)) {
        try {
          if (isQuery && dataStore.currentQuery) {
            // For SQL queries, use the cached query result
            statsPopover.histogram = await invoke<{
              bins: number[]
              counts: number[]
              min: number
              max: number
            }>('get_query_column_histogram', {
              query: dataStore.currentQuery,
              columnName,
              numBins: 20,
            })
          } else {
            // For regular files, load from file path
            statsPopover.histogram = await invoke<{
              bins: number[]
              counts: number[]
              min: number
              max: number
            }>('get_column_histogram', {
              filePath: dataStore.activeSession.path,
              columnName,
              numBins: 20,
            })
          }
        } catch (histError) {
          console.error('Error fetching histogram:', histError)
          statsPopover.histogram = null
        }
      }

      statsPopover.loading = false
    } catch (error) {
      console.error('Error fetching statistics:', error)
      statsPopover.loading = false
    }
  }

  function closeStatsPopover() {
    if (statsPopoverElement) {
      statsPopoverElement.hidePopover()
    }
    statsPopover = {
      columnIndex: null,
      stats: null,
      histogram: null,
      loading: false,
      isOpen: false,
    }
  }

  function isNumericColumn(colIndex: number): boolean {
    const dtype = columns[colIndex]?.dtype?.toLowerCase() || ''
    return (
      dtype.includes('int') ||
      dtype.includes('float') ||
      dtype.includes('double')
    )
  }

  function handleScroll() {
    if (!tableContainer) return

    const { scrollTop, scrollHeight, clientHeight } = tableContainer
    const scrollPercentage = (scrollTop + clientHeight) / scrollHeight

    // Load more when scrolled to 70%
    if (scrollPercentage > 0.7 && hasMoreRows && !isLoadingMore) {
      loadMoreRows()
    }
  }

  onMount(() => {
    // After mount and when columns rendered, measure natural header widths
    // Defer to the next microtask / paint without making the function async
    queueMicrotask(() => {
      measureAndLockColumnWidths()
    })
  })

  // Reset scroll position when switching between SQL and Raw data views
  $effect(() => {
    // We want to trigger this when isSqlTabActive changes
    dataStore.isSqlTabActive

    // untrack to avoid dependency on tableContainer if it were reactive (it is a $state)
    // but more importantly, we just want to perform the action
    untrack(() => {
      if (tableContainer) {
        tableContainer.scrollTop = 0
        tableContainer.scrollLeft = 0
      }
    })
  })

  // Re-measure when columns structure actually changes
  $effect(() => {
    // Accessing columns makes this effect run whenever they change
    const colsCount = columns.length
    const session = dataStore.activeSession

    if (colsCount === 0 || !session) {
      return
    }

    const currentColumnsJson = JSON.stringify(
      columns.map((c) => ({ name: c.name, dtype: c.dtype }))
    )

    // Use untrack to prevent reading/writing columnWidths from retriggering this effect
    untrack(() => {
      // Check against a per-session column structure marker to avoid unnecessary re-measures when switching tabs
      if (
        session.lastMeasuredColumnsJson === currentColumnsJson &&
        Object.keys(session.columnWidths).length > 0
      ) {
        return
      }

      session.lastMeasuredColumnsJson = currentColumnsJson

      // Reset layout to auto to let the browser compute natural widths when columns update
      dataStore.tableLayout = 'auto'
      // Clear previous column widths to allow re-calculation
      dataStore.columnWidths = {}

      // Defer measurement to the next microtask / paint
      queueMicrotask(() => {
        measureAndLockColumnWidths()
      })
    })
  })

  function measureAndLockColumnWidths() {
    if (!headerCells || headerCells.length === 0) return
    const newWidths: Record<number, number> = {}
    const colsCount = columns.length
    const maxWidth = window.innerWidth * MAX_COL_WIDTH_PERCENT

    for (let i = 0; i < colsCount; i++) {
      const cell = headerCells[i]
      if (!cell) continue
      // Compute content width (what <col> width applies to) to avoid double-counting paddings/borders
      const cs = getComputedStyle(cell)
      const padL = parseFloat(cs.paddingLeft || '0')
      const padR = parseFloat(cs.paddingRight || '0')
      const bordL = parseFloat(cs.borderLeftWidth || '0')
      const bordR = parseFloat(cs.borderRightWidth || '0')
      const boxW = cell.offsetWidth || MIN_COL_WIDTH
      const contentW = Math.max(0, boxW - padL - padR - bordL - bordR)
      const natural = Math.ceil(contentW)
      // Apply min and max constraints
      newWidths[i] = Math.max(MIN_COL_WIDTH, Math.min(maxWidth, natural))
    }
    if (Object.keys(newWidths).length > 0) {
      dataStore.columnWidths = newWidths
      dataStore.tableLayout = 'fixed'
    }
  }
</script>

<div class="data-table-container">
  {#if dataStore.error && dataStore.isSqlTabActive}
    <div class="error-banner">
      <div class="error-content">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <circle cx="12" cy="12" r="10" />
          <line x1="12" y1="8" x2="12" y2="12" />
          <line x1="12" y1="16" x2="12.01" y2="16" />
        </svg>
        <span class="error-label">Error:</span>
        <span class="error-message">{dataStore.error}</span>
      </div>
      <button
        class="error-dismiss"
        onclick={() => dataStore.clearError()}
        aria-label="Dismiss error"
        title="Dismiss"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <line x1="18" y1="6" x2="6" y2="18" />
          <line x1="6" y1="6" x2="18" y2="18" />
        </svg>
      </button>
    </div>
  {/if}

  {#if dataStore.isSqlTabActive && !data}
    <QueryGuide />
  {:else}
    <!-- Table container with scroll -->
    <div
      bind:this={tableContainer}
      class="table-scroll-container"
      tabindex="-1"
      onscroll={handleScroll}
    >
      <table
        class="data-table"
        class:layout-fixed={tableLayout === 'fixed'}
        class:layout-auto={tableLayout === 'auto'}
      >
        <colgroup>
          {#each columns as _, i}
            {#if columnWidths[i]}
              <col style="width: {columnWidths[i]}px;" />
            {:else}
              <col />
            {/if}
          {/each}
        </colgroup>
        <thead class="table-header">
          <tr class="header-row">
            {#each columns as column, i}
              <th
                bind:this={headerCells[i]}
                class="header-cell"
                style={`min-width:${MIN_COL_WIDTH}px`}
                onclick={(e) => handleHeaderClick(e, i)}
                oncontextmenu={(e) => handleHeaderContextMenu(e, i)}
              >
                <!-- Reference-style layout: inline flex with content and control rail.
                   Content stack has ellipsis on both name and dtype text. No borders. -->
                <div class="header-layout">
                  <!-- Content stack (flex-1) -->
                  <div class="header-content">
                    <div class="column-name">
                      <!-- svelte-ignore a11y_no_static_element_interactions -->
                      <span
                        class="column-name-text"
                        onmouseenter={(e) => tooltipStore.show(e.currentTarget, column.name)}
                        onmouseleave={() => tooltipStore.hide()}
                        >{column.name}</span
                      >
                    </div>
                    {#key column.dtype}
                      {@const IconComponent = dtypeIcon(column.dtype)}
                      <div class="column-dtype">
                        <span class="dtype-icon primary-color">
                          <IconComponent size={18} className="dtype-icon-svg" />
                        </span>
                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                        <span
                          class="dtype-text"
                          onmouseenter={(e) => tooltipStore.show(e.currentTarget, column.dtype)}
                          onmouseleave={() => tooltipStore.hide()}
                          >{column.dtype}</span
                        >
                      </div>
                    {/key}
                  </div>

                  <!-- Control rail (fixed width), vertically distributed, right-aligned -->
                  {#if !dataStore.isQueryMode}
                    <div class="control-rail">
                      <button
                        class="sort-button ascending"
                        class:sort-active={getSortDirection(column.name) ===
                          true}
                        class:sort-inactive={getSortDirection(column.name) !==
                          true}
                        title="Sort ascending"
                        onclick={() =>
                          !dataStore.isQueryMode &&
                          handleSortClick(column.name, true)}
                        disabled={dataStore.isQueryMode}
                        tabindex="-1"
                      >
                      </button>
                      {#if getSortOrder(column.name) !== null}
                        <span class="sort-order-visible"
                          >{getSortOrder(column.name)}</span
                        >
                      {:else}
                        <span class="sort-order-hidden" aria-hidden="true"
                          >0</span
                        >
                      {/if}
                      <button
                        class="sort-button descending"
                        class:sort-active={getSortDirection(column.name) ===
                          false}
                        class:sort-inactive={getSortDirection(column.name) !==
                          false}
                        title="Sort descending"
                        onclick={() =>
                          !dataStore.isQueryMode &&
                          handleSortClick(column.name, false)}
                        disabled={dataStore.isQueryMode}
                        tabindex="-1"
                      >
                      </button>
                    </div>
                  {/if}
                </div>

                <!-- Resize handle at the far right -->
                <button
                  type="button"
                  aria-label="Resize column"
                  title="Resize column"
                  class="resize-handle"
                  onmousedown={(e) => startResize(e, i)}
                  tabindex="-1"
                >
                  <span class="resize-handle-line"></span>
                </button>
              </th>
            {/each}
          </tr>
        </thead>
        <tbody>
          {#if rows.length > 0}
            {#each rows as row, index (index)}
              <tr class="data-row">
                {#each row as cell, colIndex}
                  <!-- svelte-ignore a11y_no_static_element_interactions -->
                  <td
                    class="data-cell"
                    style={`min-width:${MIN_COL_WIDTH}px`}
                    onmouseenter={(e) => tooltipStore.show(e.currentTarget, cell)}
                    onmouseleave={() => tooltipStore.hide()}
                    oncontextmenu={(e) =>
                      handleCellContextMenu(e, index, colIndex)}
                  >
                    {cell}
                  </td>
                {/each}
              </tr>
            {/each}
          {:else if !dataStore.loading && dataStore.hasData}
            <tr>
              <td colspan={columns.length || 1} class="empty-state">
                <div class="empty-state-content">
                  <span class="empty-state-title"
                    >{dataStore.isSqlTabActive
                      ? 'No SQL results'
                      : 'No data'}</span
                  >
                  {#if dataStore.isSqlTabActive && !dataStore.currentQuery}
                    <span class="empty-state-hint"
                      >Enter a query and press Run</span
                    >
                  {/if}
                </div>
              </td>
            </tr>
          {/if}

          <!-- Loading indicator -->
          {#if isLoadingMore}
            <tr>
              <td colspan={columns.length} class="loading-cell">
                <div class="loading-content">
                  <div class="loading-spinner"></div>
                  <span>Loading more rows...</span>
                </div>
              </td>
            </tr>
          {/if}
        </tbody>
      </table>
    </div>
  {/if}

  <!-- Context Menu -->
  {#if contextMenu.visible}
    <div
      bind:this={contextMenuElement}
      class="context-menu"
      style={`left: ${contextMenu.x}px; top: ${contextMenu.y}px;`}
      role="menu"
      tabindex="-1"
      onmouseleave={closeContextMenu}
    >
      {#if contextMenu.type === 'cell'}
        <button class="context-menu-item" onclick={copyValue} role="menuitem">
          Copy Value
        </button>
        <button class="context-menu-item" onclick={copyRow} role="menuitem">
          Copy Row
        </button>
        <button class="context-menu-item" onclick={copyTable} role="menuitem">
          Copy Table
        </button>
      {:else}
        <button
          class="context-menu-item"
          onclick={copyColumnName}
          role="menuitem"
        >
          Copy Column Name
        </button>
        <button
          class="context-menu-item"
          onclick={copyHeaderRow}
          role="menuitem"
        >
          Copy Header Row
        </button>
      {/if}
    </div>
  {/if}

  <!-- Stats Popover using native Popover API -->
  <div
    bind:this={statsPopoverElement}
    class="stats-popover"
    popover="auto"
    id="column-stats-popover"
  >
    {#if statsPopover.columnIndex !== null}
      {@const col = columns[statsPopover.columnIndex]}
      {@const isNumeric = isNumericColumn(statsPopover.columnIndex)}
      <div class="stats-header">
        <h3 class="stats-title">{col?.name || 'Column Statistics'}</h3>
        <button
          class="stats-close"
          onclick={closeStatsPopover}
          aria-label="Close"
        >
          <svg
            width="16"
            height="16"
            viewBox="0 0 16 16"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <line
              x1="4"
              y1="4"
              x2="12"
              y2="12"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
            />
            <line
              x1="12"
              y1="4"
              x2="4"
              y2="12"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
            />
          </svg>
        </button>
      </div>

      {#if statsPopover.loading}
        <div class="stats-loading">Loading statistics...</div>
      {:else if statsPopover.stats}
        {@const totalRows = dataStore.totalRows}
        {@const statsOrder = [
          'null_values',
          'true_count',
          'false_count',
          'min',
          'max',
          'mean',
          'percentile_25',
          'median',
          'percentile_75',
          'unique_values',
        ]}
        {@const sortedStats = statsOrder
          .filter((key) => key in statsPopover.stats)
          .map((key) => [key, statsPopover.stats[key]])}
        {@const isIntegerColumn =
          col?.dtype?.toLowerCase().includes('int') || false}
        <div class="stats-content">
          {#each sortedStats as [key, value]}
            {@const isNullValues = key === 'null_values'}
            {@const isMinMax = key === 'min' || key === 'max'}
            {@const displayValue =
              isNullValues && typeof value === 'number'
                ? (() => {
                    const pct = totalRows > 0 ? (value / totalRows) * 100 : 0
                    let pctStr
                    if (pct > 0 && pct < 0.5) {
                      pctStr = '~0.0'
                    } else if (pct >= 99.95 && pct < 100) {
                      pctStr = '~100.0'
                    } else {
                      pctStr = pct.toFixed(1)
                    }
                    return `${formatNumber(value, 0)} (${pctStr}%)`
                  })()
                : typeof value === 'number'
                  ? isIntegerColumn && isMinMax
                    ? formatNumber(value, 0)
                    : formatNumber(value, 2)
                  : value}
            <div class="stat-row">
              <span class="stat-label">{key.replace(/_/g, ' ')}:</span>
              <span class="stat-value">{displayValue}</span>
            </div>
          {/each}
        </div>

        {#if isNumeric && statsPopover.histogram && statsPopover.histogram.counts.length > 0}
          {@const hist = statsPopover.histogram}
          {@const maxCount = Math.max(...hist.counts, 1)}
          {@const bins = statsPopover.histogram.bins}
          {@const width = 280}
          {@const height = 120}
          {@const padding = 10}
          {@const barWidth = (width - 2 * padding) / hist.counts.length}
          <div class="histogram-container">
            <div class="histogram-title">Distribution</div>
            <svg viewBox={`0 0 ${width} ${height}`} class="histogram-svg">
              {#each hist.counts as count, i}
                {@const normalizedHeight = count / maxCount}
                {@const minBarHeight = 2}
                {@const barHeight =
                  count > 0
                    ? Math.max(
                        minBarHeight,
                        normalizedHeight * (height - 2 * padding)
                      )
                    : 0}
                {@const x = padding + i * barWidth}
                {@const y = height - padding - barHeight}
                <!--suppress HtmlUnknownAttribute -->
                <rect
                  {x}
                  {y}
                  width={barWidth - 1}
                  height={barHeight}
                  class="histogram-bar"
                  role="graphics-symbol"
                  aria-label={`Bin ${i + 1}: ${count} values`}
                  onmouseenter={(e) => showHistogramTooltip(e, i, count, bins)}
                  onmouseleave={hideHistogramTooltip}
                />
              {/each}
              <!-- Axes -->
              <line
                x1={padding}
                y1={height - padding}
                x2={width - padding}
                y2={height - padding}
                class="histogram-axis"
              />
              <line
                x1={padding}
                y1={padding}
                x2={padding}
                y2={height - padding}
                class="histogram-axis"
              />
            </svg>
          </div>
        {/if}
      {:else}
        <div class="stats-empty">No statistics available</div>
      {/if}
    {/if}
  </div>
</div>

<!-- Histogram Tooltip - using popover API to render in top layer -->
<div
  bind:this={histogramTooltipElement}
  class="histogram-tooltip"
  popover="manual"
>
  {histogramTooltip.text}
</div>

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<svelte:window onclick={closeContextMenu} onblur={closeStatsPopover} />

<style>
  /* Container */
  .data-table-container {
    display: flex;
    flex: 1;
    width: 100%;
    flex-direction: column;
    overflow: hidden;
    border: 1px groove var(--surface-9);
    border: 1px groove oklch(from var(--surface-9) l c h / 0.5);
    border-radius: 0.25rem;
    box-shadow: var(--default-inset);
    margin-bottom: 0.25rem;
    user-select: none;
  }

  /* Error banner */
  .error-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background-color: var(--color-error-bg);
    border-bottom: 1px solid var(--color-error);
    border-bottom: 1px solid oklch(from var(--color-error) l c h / 0.5);
    color: var(--color-error);
    padding: 1rem;
    font-size: 0.875rem;
  }

  .error-content {
    display: flex;
    align-items: start;
    gap: 0.5rem;
    flex: 1;
    margin-right: 0.5rem;
    svg {
      margin-top: 0.2rem;
    }
  }

  .error-label {
    font-weight: bold;
  }

  .error-message {
    font-family: var(--font-mono), monospace;
    user-select: text;
  }

  .error-dismiss {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.25rem;
    background: transparent;
    border: none;
    color: var(--color-error);
    cursor: pointer;
    border-radius: 0.25rem;
    transition: all 0.2s ease;
    flex-shrink: 0;
  }

  .error-dismiss:hover {
    background-color: var(--color-error-hover);
  }

  .error-dismiss:active {
    transform: scale(0.95);
  }

  /* Table scroll container */
  .table-scroll-container {
    flex: 1;
    overflow: auto;
  }

  /* Table */
  .data-table {
    font-family: var(--font-mono), monospace;
    font-size: var(--font-size-mono, 14px);
    width: 100%;
    border-collapse: collapse;
  }

  .layout-fixed {
    table-layout: fixed;
  }

  .layout-auto {
    table-layout: auto;
  }

  /* Table header */
  .table-header {
    position: sticky;
    top: 0;
    z-index: 10;
    background-color: var(--surface-3);
    /*background-image: linear-gradient(*/
    /*  to top,*/
    /*  var(--surface-2),*/
    /*  var(--surface-3)*/
    /*);*/
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.1),
      0 2px 4px -2px rgba(0, 0, 0, 0.1);
  }

  .header-cell {
    color: var(--ink-4);
    position: relative;
    padding-left: 0.5rem;
    padding-top: 0.25rem;
    padding-bottom: 0.25rem;
    height: 3rem;
    text-align: left;
    letter-spacing: 0.025em;
    &:has(.header-content:hover) {
      background-color: var(--surface-4);
    }
  }

  /* Header layout */
  .header-layout {
    position: relative;
    display: flex;
    height: 100%;
    align-items: stretch;
    border-right: 1px solid var(--surface-6);
  }

  .header-content {
    min-width: 0;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    justify-content: center;
  }

  .column-name {
    min-width: 0;
    line-height: 1.25;
    font-weight: bold;
    font-size: calc(var(--font-size-mono, 14px) + 10%);
  }

  .column-name-text {
    display: block;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .column-dtype {
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: calc(var(--font-size-mono, 14px) - 10%);
    color: var(--ink-5);
  }

  .dtype-icon {
    flex-shrink: 0;
  }

  .primary-color {
    color: var(--accent);
  }

  .dtype-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* Control rail (sort buttons) */
  .control-rail {
    flex-shrink: 0;
    width: 0.75rem;
    margin: 0 0.35rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  .sort-button {
    height: 1rem;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: transparent;
    border: none;
    padding: 0;
    cursor: pointer;
    position: relative;
  }

  .sort-button::before {
    content: '';
    position: absolute;
    inset: 0;
    background-color: var(--surface-8);
  }

  .ascending::before {
    clip-path: shape(
      from left bottom,
      line to center top,
      line to right bottom,
      arc to left bottom of 90% ccw
    );
  }

  .descending::before {
    clip-path: shape(
      from left top,
      line to center bottom,
      line to right top,
      arc to left top of 90% cw
    );
  }

  .sort-active::before {
    background-color: var(--accent);
  }

  .sort-inactive:hover::before {
    background-color: var(--ink-5);
  }

  .sort-order-visible,
  .sort-order-hidden {
    color: var(--accent);
    font-size: 0.75rem;
    font-family: 'iosevkaMP', monospace;
    height: 1rem;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    line-height: 1;
  }

  .sort-order-hidden {
    opacity: 0;
  }

  /* Resize handle */
  .resize-handle {
    position: absolute;
    top: 0;
    right: 0;
    z-index: 10;
    height: 100%;
    width: 0.25rem;
    cursor: col-resize;
    transition: colors 150ms;
    background: none;
    border: none;
    padding: 0;
  }

  .resize-handle-line {
    position: absolute;
    right: 0;
    top: 0;
    height: 100%;
    width: 1px;
    background-color: transparent;
    transition: background-color 150ms;
  }

  .resize-handle:hover .resize-handle-line {
    background-color: var(--accent);
  }

  /* Data rows */
  .data-row {
    border-bottom: 1px solid var(--surface-3);
    background-color: var(--surface-1);
    background-color: oklch(from var(--surface-1) l c h / 0.5);
    transition: background-color 150ms;
  }

  .data-row:hover {
    background-color: var(--surface-3);
  }

  .data-cell {
    color: var(--ink-2);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    padding: 0.25rem 0.5rem;
    border-right: 1px solid var(--surface-3);
  }

  /* Empty state */
  .empty-state {
    color: var(--surface-4);
    padding: 5rem 1rem;
    text-align: center;
  }

  .empty-state-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .empty-state-title {
    font-size: 1.125rem;
    font-weight: 500;
  }

  .empty-state-hint {
    font-size: 0.875rem;
  }

  /* Loading indicator */
  .loading-cell {
    padding: 1rem;
    text-align: center;
  }

  .loading-content {
    color: var(--surface-4);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .loading-spinner {
    height: 1rem;
    width: 1rem;
    border: 2px solid var(--surface-4);
    border-top-color: var(--accent);
    border-radius: 9999px;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Context Menu */
  .context-menu {
    position: fixed;
    z-index: 1000;
    background-color: var(--surface-2);
    border: 1px solid var(--surface-5);
    border-radius: 0.25rem;
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.3),
      0 2px 4px -2px rgba(0, 0, 0, 0.2);
    padding: 0.25rem;
    min-width: 150px;
  }

  .context-menu-item {
    display: block;
    width: 100%;
    padding: 0.5rem 0.75rem;
    text-align: left;
    background: none;
    border: none;
    color: var(--ink-5);
    font-size: 0.875rem;
    cursor: pointer;
    border-radius: 0.125rem;
    transition: background-color 0.15s;
  }

  .context-menu-item:hover {
    background-color: var(--surface-4);
  }

  .context-menu-item:active {
    background-color: var(--surface-5);
  }

  /* Stats Popover - native popover API */
  .stats-popover {
    position: fixed;
    background-color: var(--surface-2);
    border: 1px solid var(--surface-5);
    border-radius: 0.5rem;
    box-shadow:
      0 10px 15px -3px rgba(0, 0, 0, 0.3),
      0 4px 6px -2px rgba(0, 0, 0, 0.2);
    min-width: 320px;
    max-width: 400px;
    padding: 0;
    margin: 0;
    inset: unset;
  }

  .stats-popover:popover-open {
    animation: popover-fade-in 0.2s ease-out;
  }

  @keyframes popover-fade-in {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Reset default popover backdrop */
  .stats-popover::backdrop {
    background: transparent;
  }

  .stats-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.75rem;
    border-bottom: 1px solid var(--surface-4);
  }

  .stats-title {
    font-size: 1rem;
    font-weight: 600;
    color: var(--ink-5);
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .stats-close {
    background: none;
    border: none;
    color: var(--ink-4);
    cursor: pointer;
    padding: 0;
    width: 1.5rem;
    height: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.25rem;
    transition: all 0.15s;
  }

  .stats-close svg {
    flex-shrink: 0;
  }

  .stats-close:hover {
    background-color: var(--surface-4);
    color: var(--ink-5);
  }

  .stats-content {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
  }

  .stat-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.25rem 0;
  }

  .stat-label {
    font-size: 0.875rem;
    color: var(--ink-4);
    text-transform: capitalize;
  }

  .stat-value {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--ink-5);
    font-family: var(--font-mono), monospace;
  }

  .stats-loading,
  .stats-empty {
    padding: 2rem 1rem;
    text-align: center;
    color: var(--ink-3);
    font-size: 0.875rem;
  }

  /* Histogram */
  .histogram-container {
    padding: 1rem;
    border-top: 1px solid var(--surface-4);
  }

  .histogram-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--ink-4);
    margin-bottom: 0.75rem;
  }

  .histogram-svg {
    width: 100%;
    height: auto;
  }

  .histogram-bar {
    fill: var(--accent);
    opacity: 0.8;
    transition: opacity 0.2s;
    cursor: pointer;
  }

  .histogram-bar:hover {
    opacity: 1;
  }

  .histogram-axis {
    stroke: var(--surface-6);
    stroke-width: 1;
  }

  /* Histogram tooltip */
  .histogram-tooltip {
    position: fixed;
    transform: translate(-50%, -100%);
    background-color: var(--surface-6);
    color: var(--ink-5);
    padding: 0.5rem 0.75rem;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    font-weight: 600;
    white-space: pre-line;
    text-align: center;
    pointer-events: none;
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.3),
      0 2px 4px -2px rgba(0, 0, 0, 0.2);
    border: none;
    margin: 0;
    inset: unset;
  }

  .histogram-tooltip:popover-open {
    animation: tooltip-fade-in 0.15s ease-out;
  }

  .histogram-tooltip::backdrop {
    background: transparent;
  }

  @keyframes tooltip-fade-in {
    from {
      opacity: 0;
      transform: translate(-50%, -100%) translateY(4px);
    }
    to {
      opacity: 1;
      transform: translate(-50%, -100%);
    }
  }
</style>
