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
    } else if (dtype.value === 'Boolean') {
        return 'boolean'
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
    } else if (filterType.value === 'boolean') {
        return value.value
    } else if (filterType.value === 'text') {
        return value.value
    }

    return null
})

onMounted(() => {
    const ixInFiltering = dataStore.columnIndexInFiltering(columnName.value)
    if (
        dataStore.filtering.length &&
        ixInFiltering !== null &&
        ixInFiltering > -1
    ) {
        const filter = dataStore.filtering[ixInFiltering]
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
        } else if (filterType.value === 'boolean') {
            value.value = filter.value
        } else if (filterType.value === 'text') {
            value.value = filter.value
        }
    }
})

const emit = defineEmits(['filter', 'clear'])

async function filterData() {
    if (selectCondition.value) {
        dataStore.updateFiltering({
            column: columnName.value,
            condition: selectCondition.value,
            value: formattedValue.value,
        })
    }
    emit('filter')
}

async function clearFiltering() {
    const columnName = dataStore.columns[props.colIx].name
    dataStore.removeFilterByColumn(columnName)
    emit('clear')
}

const canBeFiltered = computed(() => {
    if (selectCondition.value === null) {
        return false
    }

    if (
        filterType.value === 'number' ||
        filterType.value === 'text' ||
        filterType.value === 'datetime'
    ) {
        return value.value !== null && value.value !== ''
    }

    if (filterType.value === 'boolean') {
        return value.value !== null
    }

    if (selectCondition.value === Condition.between) {
        return value.value !== null && value2.value !== null
    }

    return value.value !== null
})

function onEnterKey(event: KeyboardEvent) {
    if (event.key === 'Enter' && canBeFiltered.value) {
        filterData()
    }
}
</script>

<template>
    <div class="flex flex-col items-center gap-2" @keydown.enter="onEnterKey">
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
            <InputNumber
                v-model="value"
                :minFractionDigits="0"
                :maxFractionDigits="4"
                @input="value = $event.value"
            />
            <InputNumber
                v-if="selectCondition === Condition.between"
                v-model="value2"
                :minFractionDigits="0"
                :maxFractionDigits="4"
                @input="value = $event.value"
            />
        </div>
        <div v-else-if="filterType == 'boolean'" class="flex gap-1">
            <SelectButton
                v-model="value"
                :options="[
                    { label: 'True', value: true },
                    { label: 'False', value: false },
                ]"
                :option-label="(option) => option.label"
                :option-value="(option) => option.value"
                class="w-32"
            />
        </div>
        <div v-else-if="filterType == 'text'" class="flex gap-1">
            <InputText v-model="value" class="w-48" />
        </div>
        <div class="flex gap-2">
            <Button @click="filterData" :disabled="!canBeFiltered">
                Filter
            </Button>
            <Button @click="clearFiltering" outlined>Clear</Button>
        </div>
    </div>
</template>
