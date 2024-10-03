export const useTableStore = defineStore({
    id: 'tableStore',
    state: () => ({
        selectedColumn: null as Column | null,
    }),
    getters: {},
    actions: {
        selectColumn(ix: number) {
            const dataStore = useDataStore()
            if (dataStore.columns[ix] === this.selectedColumn) {
                this.selectedColumn = null
            } else {
                this.selectedColumn = dataStore.columns[ix]
            }
        },
    },
})
