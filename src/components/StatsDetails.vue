<script setup lang="ts">
const props = defineProps<{
    dtype: string
}>()

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
        v-if="numericTypes.includes(props.dtype)"
        :dtype="props.dtype"
    />
    <StatsDetailsDate v-else-if="props.dtype === 'Date'" />
    <StatsDetailsDatetime v-else-if="props.dtype === 'Datetime'" />
    <StatsDetailsTime v-else-if="props.dtype === 'Time'" />
    <StatsDetailsDuration v-else-if="props.dtype === 'Duration'" />
    <StatsDetailsOther v-else />
</template>
