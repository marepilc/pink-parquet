<script setup lang="ts">
import StatsDetails from '~/components/StatsDetails.vue'

const dataStore = useDataStore()

function formatPercentage(value: number): string {
    if (value === 1) {
        return '100%'
    } else if (value === 0) {
        return '0%'
    } else if (value < 0.005) {
        return '<1%'
    } else if (value >= 0.995 && value < 1) {
        return '~100%'
    } else {
        return `${Math.round(value * 100)}%`
    }
}

const columnHealths = computed(() => {
    return dataStore.columns.map((column) => {
        if (dataStore.noOfRows === 0) return 0
        return (
            (dataStore.noOfRows -
                dataStore.columnsInfo[column.name].null_values) /
            dataStore.noOfRows
        )
    })
})
</script>

<template>
    <div class="flex w-full flex-col items-center justify-between bg-cyan-100">
        <h2 class="text-lg font-semibold">Data health</h2>

        <!-- Bar Chart -->
        <div class="flex w-full gap-2">
            <div
                v-for="(colValue, ix) in columnHealths"
                :key="ix"
                class="flex min-w-0 flex-1 basis-0 flex-col items-center"
            >
                <div class="relative h-32 w-full bg-red-500">
                    <div
                        :style="{
                            height: colValue * 100 + '%',
                            backgroundColor: '#3b82f6',
                        }"
                        class="absolute bottom-0 left-0 w-full"
                    ></div>
                </div>
            </div>
        </div>

        <div class="flex w-full gap-2">
            <div
                v-for="(colHealth, ix) in columnHealths"
                :key="ix"
                class="min-w-0 flex-1 basis-0 bg-amber-200 text-center"
            >
                {{ formatPercentage(colHealth) }}
            </div>
        </div>
    </div>
</template>

<style scoped></style>
