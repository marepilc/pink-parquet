<script setup lang="ts">
const props = defineProps<{
    dtype: string
}>()
</script>

<template>
    <div>
        <div v-if="props.dtype == 'String'">
            <IconString />
        </div>
        <div v-else-if="props.dtype == 'Boolean'">
            <IconBoolean />
        </div>
        <div v-else-if="props.dtype == 'Binary'">
            <IconBinary />
        </div>
        <div v-else-if="props.dtype == 'Datetime'">
            <IconDatetime />
        </div>
        <div v-else-if="props.dtype == 'Time'">
            <IconTime />
        </div>
        <div v-else-if="props.dtype == 'Duration'">
            <IconDuration />
        </div>
        <div v-else-if="props.dtype == 'List'">
            <IconList />
        </div>
        <div v-else-if="props.dtype == 'Struct'">
            <IconStruct />
        </div>
        <div v-else-if="props.dtype == 'Categorical'">
            <IconCategorical />
        </div>
        <div v-else-if="props.dtype.includes('Int')">
            <IconInteger />
        </div>
        <div v-else-if="props.dtype.startsWith('Float')">
            <IconFloat />
        </div>
        <div v-else-if="props.dtype == 'Date'">
            <IconCalendar />
        </div>
    </div>
</template>
