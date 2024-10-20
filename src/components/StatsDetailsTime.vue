<script setup lang="ts">
const tableStore = useTableStore()
const dataStore = useDataStore()
</script>

<template>
    <div class="flex flex-col">
        <div>
            min:
            {{
                convertToTime(
                    dataStore.columnsInfo[tableStore.selectedColumn.name]['min']
                )
            }}
        </div>
        <div>
            max:
            {{
                convertToTime(
                    dataStore.columnsInfo[tableStore.selectedColumn.name]['max']
                )
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
