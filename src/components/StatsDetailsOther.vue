<script setup lang="ts">
import { bytesForHuman } from '~/utils/format'

const props = defineProps<{
    dtype: string
}>()

const tableStore = useTableStore() as any
const dataStore = useDataStore() as any

const uniqueValues = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name][
        'unique_values'
    ]
})

const nullsValue = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name]['null_values']
})

const memoryDemand = computed(() => {
    switch (props.dtype) {
        case 'Boolean':
            return bytesForHuman(dataStore.noOfRows)
        default:
            return 'N/A'
    }
})
</script>

<template>
    <div class="flex gap-2">
        <div class="grid min-w-32 grid-cols-2 gap-x-2 gap-y-0">
            <div class="text-right">unique values:</div>
            <div class="truncate font-semibold">
                {{ uniqueValues }}
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
