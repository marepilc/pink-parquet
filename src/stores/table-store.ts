import type { Column } from '~/types/app-types'

export const useTableStore = defineStore({
    id: 'tableStore',
    state: () => ({
        selectedColumn: null as Column | null,
    }),
    getters: {
        filterDisabled: () => (column: Column) => {
            const supportedTypes = new Set([
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
                'Boolean',
                'String',
                'Date',
                'Datetime',
                'Categorical',
            ])

            return !supportedTypes.has(column.dtype)
        },
    },
    actions: {
        selectColumn(ix: number) {
            const dataStore = useDataStore()
            if (dataStore.columns[ix] === this.selectedColumn) {
                this.selectedColumn = null
            } else {
                this.selectedColumn = dataStore.columns[ix]
            }
        },
        deselectColumn() {
            this.selectedColumn = null
        },
    },
})
