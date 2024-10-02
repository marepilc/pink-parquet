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
        rows: [] as string[][],
        noOfRows: 0,
        noOfColumns: 0,
        rowsLoadingCounter: 0,
        rowsLoadingInProgress: false,
        sorting: [] as Sorting[],
    }),
    getters: {},
    actions: {
        async loadParquet(filePath: string) {
            try {
                const data = await invoke<ParquetData>('get_data', { filePath })
                console.log(data)
                if (this.isFileOpen) {
                    this.resetContent()
                }
                this.updateOpenState(true, filePath, data.shape)
                this.updateColumns(
                    data.columns.map((col) => ({
                        ...col,
                        dtype: dtypeCleaner(col.dtype),
                    }))
                )
                this.addRows(data.rows)
                this.updateDraggedFilePath(null)
            } catch (e) {
                console.error(e)
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
            this.rows.push(...rows)
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
    },
})
