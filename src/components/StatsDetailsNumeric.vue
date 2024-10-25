<script setup lang="ts">
import { bytesForHuman } from '~/utils/format'

const props = defineProps<{
    dtype: string
}>()

const tableStore = useTableStore() as any
const dataStore = useDataStore() as any

const minValue = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name]['min']
})

const maxValue = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name]['max']
})

const meanValue = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name]['mean']
})

const percentile25Value = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name][
        'percentile_25'
    ]
})

const medianValue = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name]['median']
})

const percentile75Value = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name][
        'percentile_75'
    ]
})

const nullsValue = computed(() => {
    return dataStore.columnsInfo[tableStore.selectedColumn.name]['null_values']
})

const memoryDemand = computed(() => {
    switch (props.dtype) {
        case 'Int8':
            return bytesForHuman(dataStore.noOfRows)
        case 'Int16':
            return bytesForHuman(dataStore.noOfRows * 2)
        case 'Int32':
            return bytesForHuman(dataStore.noOfRows * 4)
        case 'Int64':
            return bytesForHuman(dataStore.noOfRows * 8)
        case 'Float32':
            return bytesForHuman(dataStore.noOfRows * 4)
        case 'Float64':
            return bytesForHuman(dataStore.noOfRows * 8)
        case 'UInt8':
            return bytesForHuman(dataStore.noOfRows)
        case 'UInt16':
            return bytesForHuman(dataStore.noOfRows * 2)
        case 'UInt32':
            return bytesForHuman(dataStore.noOfRows * 4)
        case 'UInt64':
            return bytesForHuman(dataStore.noOfRows * 8)
        default:
            return 'N/A'
    }
})

// computed property to check optimal dtype to store the column data
const optimalDtype = computed(() => {
    if (props.dtype.startsWith('Float')) {
        if (minValue.value >= -3.4e38 && maxValue.value <= 3.4e38) {
            return 'Float32'
        } else {
            return 'Float64'
        }
    } else {
        if (minValue.value >= -128 && maxValue.value <= 127) {
            return 'Int8'
        } else if (minValue.value >= 0 && maxValue.value <= 255) {
            return 'UInt8'
        } else if (minValue.value >= -32768 && maxValue.value <= 32767) {
            return 'Int16'
        } else if (minValue.value >= 0 && maxValue.value <= 65535) {
            return 'UInt16'
        } else if (
            minValue.value >= -2147483648 &&
            maxValue.value <= 2147483647
        ) {
            return 'Int32'
        } else if (minValue.value >= 0 && maxValue.value <= 4294967295) {
            return 'UInt32'
        } else if (minValue.value >= 0) {
            return 'UInt64'
        } else {
            return 'Int64'
        }
    }
})

const optimalDtypeMemoryDemand = computed(() => {
    switch (optimalDtype.value) {
        case 'Int8':
            return bytesForHuman(dataStore.noOfRows)
        case 'Int16':
            return bytesForHuman(dataStore.noOfRows * 2)
        case 'Int32':
            return bytesForHuman(dataStore.noOfRows * 4)
        case 'Int64':
            return bytesForHuman(dataStore.noOfRows * 8)
        case 'Float32':
            return bytesForHuman(dataStore.noOfRows * 4)
        case 'Float64':
            return bytesForHuman(dataStore.noOfRows * 8)
        case 'UInt8':
            return bytesForHuman(dataStore.noOfRows)
        case 'UInt16':
            return bytesForHuman(dataStore.noOfRows * 2)
        case 'UInt32':
            return bytesForHuman(dataStore.noOfRows * 4)
        case 'UInt64':
            return bytesForHuman(dataStore.noOfRows * 8)
        default:
            return 'N/A'
    }
})

const isDtypeOptimal = computed(() => {
    return props.dtype === optimalDtype.value
})

const memoryDemandPercentage = computed(() => {
    if (memoryDemand.value === 'N/A') return 'N/A'
    return (
        (parseInt(optimalDtypeMemoryDemand.value) /
            parseInt(memoryDemand.value)) *
        100
    ).toFixed(2)
})
</script>

<template>
    <div class="flex gap-2">
        <div class="grid min-w-60 grid-cols-2 gap-x-2 gap-y-0">
            <div class="text-right">min:</div>
            <div class="truncate font-semibold">
                {{ minValue }}
            </div>
            <div class="text-right">max:</div>
            <div class="truncate font-semibold">
                {{ maxValue }}
            </div>
            <div class="text-right">mean:</div>
            <div class="truncate font-semibold">
                {{ meanValue }}
            </div>
            <div class="text-right">25th percentile:</div>
            <div class="truncate font-semibold">
                {{ percentile25Value }}
            </div>
            <div class="text-right">median:</div>
            <div class="truncate font-semibold">
                {{ medianValue }}
            </div>
            <div class="text-right">75th percentile:</div>
            <div class="truncate font-semibold">
                {{ percentile75Value }}
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
                <span class="font-semibold">{{ memoryDemand }}</span>
            </div>
            <div v-if="!isDtypeOptimal">
                Values within this range can be stored using the
                <span class="font-semibold">{{ optimalDtype }}</span>
                data type. The required memory would be
                <span class="font-semibold">
                    {{ optimalDtypeMemoryDemand }}
                </span>
                , which is
                <span class="font-semibold">{{ memoryDemandPercentage }}%</span>
                of the memory needed for the current data type.
            </div>
        </div>
    </div>
</template>
