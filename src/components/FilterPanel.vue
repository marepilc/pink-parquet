<script setup lang="ts">
import { Condition } from '~/types/app-types'

import { formatDate } from '~/utils/format'

const props = defineProps<{
    colIx: number
}>()

const dataStore = useDataStore()

const dtype = computed(() => dataStore.columns[props.colIx].dtype)
const columnName = computed(() => dataStore.columns[props.colIx].name)

const filterType = computed(() => {
    if (dtype.value === 'Date' || dtype.value === 'Datetime') {
        return 'datetime'
    } else if (dtype.value === 'Time') {
        return 'time'
    } else if (dtype.value === 'Duration') {
        return 'duration'
    } else if (
        [
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
        ].includes(dtype.value)
    ) {
        return 'number'
    } else {
        return 'text'
    }
})

const selectCondition = ref<Condition | null>(null)
const value = ref(null)
const value2 = ref(null)

function getFormattedValue(date: Date, format: string): string {
    // Default format for 'Date'
    const defaultFormat = 'yyyy-MM-dd'
    return formatDate(date, format || defaultFormat)
}

const formattedValue = computed(() => {
    if (filterType.value === 'datetime' && value.value) {
        const format =
            dtype.value === 'Datetime'
                ? 'yyyy-MM-ddTHH:mm:ss.SSS'
                : 'yyyy-MM-dd'

        if (selectCondition.value !== Condition.between) {
            // For single value (Date or Datetime)
            return getFormattedValue(new Date(value.value), format)
        } else if (
            selectCondition.value === Condition.between &&
            value2.value
        ) {
            // For range of two values (between condition)
            const date1 = new Date(value.value)
            const date2 = new Date(value2.value)

            return [
                getFormattedValue(date1, format),
                getFormattedValue(date2, format),
            ]
        }
    } else if (filterType.value === 'number') {
        if (selectCondition.value !== Condition.between) {
            return value.value
        } else if (
            selectCondition.value === Condition.between &&
            value2.value
        ) {
            return [value.value, value2.value]
        }
    }

    return null
})

onMounted(() => {
    const ixInFiltering = dataStore.columnIndexInFiltering(columnName.value)
    if (dataStore.filtering.length && ixInFiltering > -1) {
        const filter =
            dataStore.filtering[
                dataStore.columnIndexInFiltering(columnName.value)
            ]
        selectCondition.value = filter.condition
        if (filterType.value === 'datetime') {
            // parse date from value to model value
            const dateRegex = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}$/
            if (selectCondition.value !== Condition.between) {
                if (dateRegex.test(filter.value)) {
                    value.value = new Date(filter.value + 'Z')
                }
            } else {
                const [date1, date2] = filter.value
                if (dateRegex.test(date1) && dateRegex.test(date2)) {
                    value.value = new Date(date1 + 'Z')
                    value2.value = new Date(date2 + 'Z')
                }
            }
        } else if (filterType.value === 'number') {
            if (selectCondition.value !== Condition.between) {
                value.value = filter.value
            } else {
                const [val1, val2] = filter.value
                value.value = val1
                value2.value = val2
            }
        }
    }
})

const emit = defineEmits(['close'])

async function filterData() {
    if (selectCondition.value) {
        dataStore.updateFiltering({
            column: columnName.value,
            condition: selectCondition.value,
            value: formattedValue.value,
        })
        await dataStore.loadParquet(dataStore.filePath)
    }
    //emit close event
    emit('close')
}

async function clearFiltering() {
    dataStore.filtering = []
    await dataStore.loadParquet(dataStore.filePath)
    emit('close')
}

const canBeFiltered = computed(() => {
    return !selectCondition.value || !formattedValue.value
})
</script>

<template>
    <div class="flex flex-col items-center gap-2">
        <FilterConditions :dtype="dtype" v-model="selectCondition" />
        <div v-if="filterType == 'datetime'" class="flex gap-1">
            <DatePicker class="w-32" v-model="value" dateFormat="yy-mm-dd" />
            <DatePicker
                v-if="selectCondition === Condition.between"
                v-model="value2"
                dateFormat="yy-mm-dd"
                class="w-32"
            />
        </div>
        <div v-else-if="filterType == 'number'" class="flex gap-1">
            <InputNumber v-model="value" />
            <InputNumber
                v-if="selectCondition === Condition.between"
                v-model="value2"
            />
        </div>
        <div class="flex gap-2">
            <Button @click="filterData" :disabled="canBeFiltered">
                Filter
            </Button>
            <Button @click="clearFiltering" outlined>Clear</Button>
        </div>
    </div>
</template>

<style scoped></style>
