<script setup lang="ts">
const tableStore = useTableStore()
const dataStore = useDataStore()

const numericTypes = [
    'Int8',
    'Int16',
    'Int32',
    'Int64',
    'Float32',
    'Float64',
    'UInt8',
    'UInt16',
    'UInt32',
    'UInt64',
]
</script>

<template>
    <StatsDetailsNumeric
        v-if="numericTypes.includes(tableStore.selectedColumn.dtype)"
    />
    <StatsDetailsDate v-else-if="tableStore.selectedColumn.dtype === 'Date'" />
    <StatsDetailsDatetime
        v-else-if="tableStore.selectedColumn.dtype === 'Datetime'"
    />
    <StatsDetailsTime v-else-if="tableStore.selectedColumn.dtype === 'Time'" />
    <StatsDetailsDuration
        v-else-if="tableStore.selectedColumn.dtype === 'Duration'"
    />
    <StatsDetailsOther v-else />
</template>
