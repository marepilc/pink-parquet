import type {
    Column,
    FileMetadata,
    Filtering,
    Shape,
    Sorting,
} from '~/types/app-types'
import { invoke } from '@tauri-apps/api/core'
import { dtypeCleaner } from '~/utils/dtype-cleaner'
import { Condition } from '~/types/app-types'

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

                this.updateOpenState(true, filePath, data.shape, data.height)
                this.updateFileMetadata(data.metadata)
                this.updateColumns(
                    data.columns.map((col) => ({
                        ...col,
                        dtype: dtypeCleaner(col.dtype),
                    }))
                )
                this.addRows(data.rows)
                if (!sorting) {
                    this.updateDraggedFilePath(null)
                }
            } catch (e) {
                console.error(e)
            }
        },
        async getMoreRows() {
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
                    this.addRows(data)
                } else {
                    console.error('Data is not an array')
                }
            } catch (e) {
                console.error(e)
            } finally {
                this.rowsLoadingInProgress = false
            }
        },
        updateOpenState(
            isOpen: boolean,
            filePath: string,
            shape: Shape,
            height: number
        ) {
            this.isFileOpen = isOpen
            this.filePath = filePath
            this.noOfRows = shape[0]
            this.noOfColumns = shape[1]
            this.filteredRowsCount = height
        },
        updateFileMetadata(metadata: FileMetadata | null) {
            if (metadata === null) {
                this.fileMetadata = metadata
            } else {
                this.fileMetadata = {
                    fileName: metadata.file_name,
                    createdAt: metadata.created_at,
                    modifiedAt: metadata.modified_at,
                    size: metadata.file_size as number,
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
    },
})
