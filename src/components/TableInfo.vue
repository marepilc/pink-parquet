<script setup lang="ts">
const dataStore = useDataStore()
const tableStore = useTableStore()

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
    <div class="flex w-full flex-col items-center justify-between">
        <h2 class="text-lg font-semibold">Data health</h2>

        <!-- Bar Chart -->
        <div class="flex w-full gap-1">
            <div
                v-for="(colValue, ix) in columnHealths"
                :key="ix"
                class="flex min-w-8 flex-1 basis-0 flex-col items-center"
            >
                <div
                    class="relative h-32 w-full bg-primary"
                    v-tooltip.top="{
                        value: `${dataStore.columns[ix].name} (${dataStore.columns[ix].dtype})`,
                        showDelay: 250,
                    }"
                    @click="tableStore.selectColumn(ix)"
                >
                    <div
                        :style="{
                            height: colValue * 100 + '%',
                        }"
                        class="absolute bottom-0 left-0 w-full bg-surface-400 dark:bg-surface-600"
                    ></div>
                </div>
            </div>
        </div>

        <div class="flex w-full gap-1">
            <div
                v-for="(colHealth, ix) in columnHealths"
                :key="ix"
                class="min-w-8 flex-1 basis-0 text-center text-sm"
                :class="{
                    'font-semibold text-primary': colHealth < 1,
                }"
            >
                {{ formatPercentage(colHealth) }}
            </div>
        </div>
    </div>
</template>

<style scoped></style>
