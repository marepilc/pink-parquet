<script setup lang="ts">
const tableStore = useTableStore()
const dataStore = useDataStore()
</script>

<template>
    <div class="flex flex-col">
        <div>
            unique values:
            {{
                dataStore.columnsInfo[tableStore.selectedColumn.name][
                    'unique_values'
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
