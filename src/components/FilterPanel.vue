<script setup lang="ts">
import { Condition } from '~/types/app-types'

const props = defineProps<{
    colIx: number
}>()

const dataStore = useDataStore()

const dtype = computed(() => dataStore.columns[props.colIx].dtype)

const selectCondition = ref<Condition | null>(null)
const value = ref(null)

const formattedValue = computed(() => {
    if (dtype.value === 'Date' && value.value) {
        return new Date(value.value).toISOString().slice(0, -1)
    }
    return null
})

const emit = defineEmits(['close'])

async function filterData() {
    if (selectCondition.value) {
        console.log('filtering', selectCondition.value, formattedValue.value)
        dataStore.filtering = [
            {
                column: 'date',
                condition: selectCondition.value,
                value: formattedValue.value,
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

const dateValue = ref(new Date())
</script>

<template>
    <div class="flex flex-col gap-2">
        <FilterConditions :dtype="dtype" v-model="selectCondition" />
        <div v-if="dtype == 'Date'">
            <PpDatePicker v-model="value" dateFormat="yy-mm-dd" />
        </div>
        <div class="flex gap-2">
            <Button @click="filterData">Filter</Button>
            <Button @click="clearFiltering" outlined>Clear</Button>
        </div>
    </div>
</template>

<style scoped></style>
