import type {
    Column,
    FileMetadata,
    Filtering,
    Shape,
    Sorting,
    ParquetData,
} from '~/types/app-types'
import { invoke } from '@tauri-apps/api/core'
import { dtypeCleaner } from '~/utils/dtype-cleaner'

export const useDataStore = defineStore({
    id: 'dataStore',
    state: () => ({
        isFileOpen: false,
        filePath: '',
        fileMetadata: null as FileMetadata | null,
        draggedFilePath: null as string | null,
        columns: [] as Column[],
        columnsInfo: {} as { [key: string]: {} },
        rows: shallowRef<string[][]>([]),
        noOfRows: 0,
        filteredRowsCount: 0,
        noOfColumns: 0,
        rowsLoadingCounter: 0,
        rowsLoadingInProgress: false,
        sorting: [] as Sorting[],
        filtering: [] as Filtering[],
        loadingInProgress: false,
    }),
    getters: {
        allRowsLoaded: (state) => {
            return state.rowsLoadingCounter * 250 >= state.noOfRows
        },
        indexInSorting: (state) => {
            return (srt: Sorting) => {
                const index = state.sorting.findIndex(
                    (s) =>
                        s.column === srt.column && s.ascending === srt.ascending
                )
                return index !== -1 ? index + 1 : null
            }
        },
        columnIndexInSorting: (state) => {
            return (column: string) => {
                const ix = state.sorting.findIndex((s) => s.column === column)
                return ix !== -1 ? ix + 1 : null
            }
        },
        columnIndexInFiltering: (state) => {
            return (column: string) => {
                const ix = state.filtering.findIndex((f) => f.column === column)
                return ix !== -1 ? ix : null
            }
        },
        columnInFiltering: (state) => {
            return (column: string) => {
                const ix = state.filtering.findIndex((f) => f.column === column)
                return ix !== -1
            }
        },
    },
    actions: {
        async loadParquet(filePath: string) {
            this.loadingInProgress = true
            const sorting =
                this.sorting && this.sorting.length > 0 ? this.sorting : null
            const filtering =
                this.filtering && this.filtering.length > 0
                    ? this.filtering
                    : null
            try {
                const data = await invoke<ParquetData>('get_data', {
                    filePath,
                    sorting,
                    filtering,
                })

                this.columnsInfo = await invoke('get_statistics', { filePath })

                this.resetContent(false)

                this.updateOpenState(true, filePath, data.shape)
                this.updateFileMetadata(data.metadata)
                this.updateColumns(
                    data.columns.map((col: Column) => ({
                        ...col,
                        dtype: dtypeCleaner(col.dtype),
                    }))
                )
                this.addRows(this.formatRows(data.rows))
                if (!sorting) {
                    this.updateDraggedFilePath(null)
                }
                this.loadingInProgress = false
            } catch (e) {
                console.error(e)
            }
        },
        async getMoreRows() {
            this.loadingInProgress = true
            if (this.rowsLoadingInProgress) return
            this.rowsLoadingInProgress = true
            try {
                const sorting =
                    this.sorting && this.sorting.length > 0
                        ? this.sorting
                        : null
                const filtering =
                    this.filtering && this.filtering.length > 0
                        ? this.filtering
                        : null
                const data = await invoke('get_more_rows', {
                    filePath: this.filePath,
                    offset: 250 * this.rowsLoadingCounter,
                    limit: 250,
                    sorting: sorting,
                    filtering: filtering,
                })

                if (Array.isArray(data)) {
                    this.addRows(this.formatRows(data))
                } else {
                    console.error('Data is not an array')
                }
                this.loadingInProgress = false
            } catch (e) {
                console.error(e)
            } finally {
                this.rowsLoadingInProgress = false
            }
        },
        updateOpenState(isOpen: boolean, filePath: string, shape: Shape) {
            this.isFileOpen = isOpen
            this.filePath = filePath
            this.noOfColumns = shape[1]
            this.filteredRowsCount = shape[0]
        },
        async updateFileMetadata(metadata: FileMetadata | null) {
            if (metadata === null) {
                this.fileMetadata = null
            } else {
                this.fileMetadata = {
                    fileName: metadata.fileName,
                    createdAt: metadata.createdAt,
                    modifiedAt: metadata.modifiedAt,
                    fileSize: metadata.fileSize as number,
                    numRows: metadata.numRows,
                    columns: metadata.columns,
                }
                this.noOfRows = metadata.numRows

                await nextTick()
                for (let i = 0; i < this.columns.length; i++) {
                    this.columns[i].compression =
                        metadata.columns[i].compression
                }
            }
        },
        updateColumns(columns: Column[]) {
            this.columns = columns
        },
        addRows(rows: string[][]) {
            this.rows = [...this.rows, ...rows]
            this.rowsLoadingCounter++
        },
        resetContent(resetSortingAndFiltering = true) {
            this.columns = []
            this.rows = []
            this.noOfRows = 0
            this.filteredRowsCount = 0
            this.noOfColumns = 0
            this.rowsLoadingCounter = 0
            this.rowsLoadingInProgress = false
            this.fileMetadata = null
            if (resetSortingAndFiltering) {
                this.sorting = []
                this.filtering = []
            }
        },
        updateDraggedFilePath(filePath: string | null) {
            this.draggedFilePath = filePath
        },
        updateSorting(sortingRequest: Sorting) {
            const index = this.sorting.findIndex(
                (s) => s.column === sortingRequest.column
            )
            if (index === -1) {
                this.sorting.push(sortingRequest)
            } else if (
                this.sorting[index].ascending !== sortingRequest.ascending
            ) {
                this.sorting[index].ascending = !this.sorting[index].ascending
            } else {
                // Remove sorting if the same column is clicked twice
                this.sorting.splice(index, 1)
            }
        },
        clearSorting() {
            this.sorting = []
        },
        updateFiltering(filteringRequest: Filtering) {
            const index = this.filtering.findIndex(
                (f) => f.column === filteringRequest.column
            )
            if (index === -1) {
                this.filtering.push(filteringRequest)
            } else {
                this.filtering[index] = filteringRequest
            }
        },
        clearFiltering() {
            this.filtering = []
        },
        removeFilterByColumn(columnName: string) {
            const index = this.filtering.findIndex(
                (f) => f.column === columnName
            )
            if (index !== -1) {
                this.filtering.splice(index, 1)
            }
        },
        formatRows(rows: string[][]): string[][] {
            // Get the indices of numeric columns based on dtype
            const numericColumns = this.columns
                .map((col, index) => {
                    const isNumeric = [
                        'Int8',
                        'Int16',
                        'Int32',
                        'Int64',
                        'UInt8',
                        'UInt16',
                        'UInt32',
                        'UInt64',
                        'Float32',
                        'Float64',
                    ].includes(col.dtype)
                    return isNumeric ? index : null
                })
                .filter((index): index is number => index !== null)

            const { intLengths, fracLengths } = this.calculateMaxPartsLengths(
                rows,
                numericColumns
            )

            return rows.map((row) =>
                row.map((value, colIndex) => {
                    if (
                        numericColumns.includes(colIndex) &&
                        !isNaN(Number(value))
                    ) {
                        // Format numeric columns with decimal alignment
                        const numericIndex = numericColumns.indexOf(colIndex)
                        const [integerPart, fractionalPart = ''] =
                            value.split('.')
                        const paddedInt =
                            ' '.repeat(
                                intLengths[numericIndex] - integerPart.length
                            ) + integerPart
                        const paddedFrac =
                            fractionalPart +
                            ' '.repeat(
                                fracLengths[numericIndex] -
                                    fractionalPart.length
                            )
                        return fracLengths[numericIndex] > 0
                            ? `${paddedInt}.${paddedFrac}`
                            : paddedInt
                    }
                    // Return other columns unchanged
                    return value
                })
            )
        },
        calculateMaxPartsLengths(
            rows: string[][],
            numericColumns: number[]
        ): {
            intLengths: number[]
            fracLengths: number[]
        } {
            const intLengths = numericColumns.map((colIndex) =>
                Math.max(
                    ...rows.map((row) => {
                        const parts = row[colIndex].split('.')
                        return parts[0].length
                    })
                )
            )

            const fracLengths = numericColumns.map((colIndex) =>
                Math.max(
                    ...rows.map((row) => {
                        const parts = row[colIndex].split('.')
                        return parts.length > 1 ? parts[1].length : 0
                    })
                )
            )

            return { intLengths, fracLengths }
        },
    },
})
