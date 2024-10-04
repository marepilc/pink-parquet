import type { Column, Shape, Sorting } from '~/types/app-types'
import { invoke } from '@tauri-apps/api/core'
import { dtypeCleaner } from '~/utils/dtype-cleaner'

export const useDataStore = defineStore({
    id: 'dataStore',
    state: () => ({
        isFileOpen: false,
        filePath: '',
        draggedFilePath: null as string | null,
        columns: [] as Column[],
        rows: shallowRef<string[][]>([]),
        noOfRows: 0,
        noOfColumns: 0,
        rowsLoadingCounter: 0,
        rowsLoadingInProgress: false,
        sorting: [] as Sorting[],
    }),
    getters: {
        allRowsLoaded: (state) => {
            return state.rowsLoadingCounter * 100 >= state.noOfRows
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
    },
    actions: {
        async loadParquet(filePath: string) {
            const sorting =
                this.sorting && this.sorting.length > 0 ? this.sorting : null
            try {
                const data = await invoke<ParquetData>('get_data', {
                    filePath,
                    sorting,
                })
                console.log(data)
                if (this.isFileOpen) {
                    this.resetContent(false)
                }
                this.updateOpenState(true, filePath, data.shape)
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
                const data = await invoke('get_more_rows', {
                    filePath: this.filePath,
                    offset: 100 * this.rowsLoadingCounter,
                    limit: 100,
                    sorting: sorting,
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
        updateOpenState(isOpen: boolean, filePath: string, shape: Shape) {
            this.isFileOpen = isOpen
            this.filePath = filePath
            this.noOfRows = shape[0]
            this.noOfColumns = shape[1]
        },
        updateColumns(columns: Column[]) {
            this.columns = columns
        },
        addRows(rows: string[][]) {
            this.rows = [...this.rows, ...rows]
            this.rowsLoadingCounter++
        },
        resetContent(resetSorting = true) {
            this.columns = []
            this.rows = []
            this.noOfRows = 0
            this.noOfColumns = 0
            this.rowsLoadingCounter = 0
            this.rowsLoadingInProgress = false
            if (resetSorting) {
                this.sorting = []
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
    },
})
