<script setup lang="ts">
import { Condition } from '~/types/app-types'

const props = defineProps<{
    columnName: string
    dtype: string
}>()

const dataStore = useDataStore()

const selectCondition = ref<Condition | null>(null)
const value = ref<string | number | [string, string]>('')

onMounted(() => {
    dataStore.filtering.forEach((filter) => {
        if (filter.column === props.columnName) {
            selectCondition.value = filter.condition
            value.value = filter.value
            console.log('filter', filter)
        }
    })
})

const emit = defineEmits(['close'])

async function filterData() {
    if (selectCondition.value) {
        const formattedValue = new Date(value.value).toISOString().slice(0, -1)
        dataStore.filtering = [
            {
                column: 'date',
                condition: selectCondition.value,
                value: formattedValue,
            },
        ]
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
</script>

<template>
    <div class="flex flex-col gap-2">
        <FilterConditions :dtype="props.dtype" v-model="selectCondition" />
        <div v-if="dtype == 'Date'">
            <UInput v-model="value" />
        </div>
        <div class="flex gap-2">
            <UButton @click="filterData">Filter</UButton>
            <UButton @click="clearFiltering">Clear</UButton>
        </div>
    </div>
</template>

<style scoped></style>
