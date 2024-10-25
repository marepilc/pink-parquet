<script setup lang="ts">
const tableStore = useTableStore() as any
const dataStore = useDataStore() as any

const minValue = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name]['min']
})

const maxValue = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name]['max']
})

const nullsValue = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name]['null_values']
})

const memoryDemand = computed(() => {
    return bytesForHuman(dataStore.noOfRows * 8)
})
</script>

<template>
    <div class="flex gap-2">
        <div class="grid min-w-32 grid-cols-2 gap-x-2 gap-y-0">
            <div class="text-right">min:</div>
            <div class="truncate font-semibold">
                {{ convertToDuration(minValue) }}
            </div>
            <div class="text-right">max:</div>
            <div class="truncate font-semibold">
                {{ convertToDuration(maxValue) }}
            </div>
            <div class="text-right">nulls:</div>
            <div class="truncate font-semibold">
                {{ nullsValue }}
            </div>
        </div>
        <Divider layout="vertical" />
        <div class="flex flex-col gap-2">
            <div>
                Memory:
                <span class="font-semibold">
                    {{ memoryDemand }}
                </span>
            </div>
        </div>
    </div>
</template>
