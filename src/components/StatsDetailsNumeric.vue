<script setup lang="ts">
const tableStore = useTableStore()
const dataStore = useDataStore()
</script>

<template>
    <div class="flex flex-col">
        <div>
            min:
            {{ dataStore.columnsInfo[tableStore.selectedColumn.name]['min'] }}
        </div>
        <div>
            max:
            {{ dataStore.columnsInfo[tableStore.selectedColumn.name]['max'] }}
        </div>
        <div>
            mean:
            {{ dataStore.columnsInfo[tableStore.selectedColumn.name]['mean'] }}
        </div>
        <div>
            25th percentile:
            {{
                dataStore.columnsInfo[tableStore.selectedColumn.name][
                    'percentile_25'
                ]
            }}
        </div>
        <div>
            median:
            {{
                dataStore.columnsInfo[tableStore.selectedColumn.name]['median']
            }}
        </div>
        <div>
            75th percentile:
            {{
                dataStore.columnsInfo[tableStore.selectedColumn.name][
                    'percentile_75'
                ]
            }}
        </div>
        <div>
            nulls:
            {{
                dataStore.columnsInfo[tableStore.selectedColumn.name][
                    'null_values'
                ]
            }}
        </div>
    </div>
</template>

<style scoped></style>
