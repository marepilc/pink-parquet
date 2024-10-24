<script setup lang="ts">
import type { Column } from '~/types/app-types'

const dataStore = useDataStore() as any
const tableStore = useTableStore() as any

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
    return dataStore.columns.map((column: Column) => {
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
    <div
        class="flex w-full max-w-full flex-col items-center justify-between overflow-hidden px-2"
    >
        <h2
            class="overflow-hidden text-ellipsis whitespace-nowrap text-lg font-semibold uppercase"
        >
            &#128151; Data health
        </h2>

        <!-- Shared Scrollable Container for Bar Chart and Values -->
        <div class="scrollbar-custom w-full overflow-auto">
            <div class="flex gap-2">
                <!-- Bar Chart Columns -->
                <div
                    v-for="(colValue, ix) in columnHealths"
                    :key="ix"
                    class="flex min-w-8 flex-grow flex-col items-center"
                >
                    <div
                        class="relative h-32 w-full bg-primary"
                        v-tooltip.top="{
                            value: `${dataStore.columns[ix].name} (${dataStore.columns[ix].dtype})`,
                            showDelay: 1000,
                        }"
                        @click="tableStore.selectColumn(ix)"
                    >
                        <div
                            :style="{ height: colValue * 100 + '%' }"
                            class="absolute bottom-0 left-0 w-full bg-surface-400 dark:bg-surface-600"
                        ></div>
                        <!-- Correctly Rotated Text at Bottom of the Column -->
                        <div
                            class="absolute bottom-16 left-1/2 ml-2.5 w-[110px] origin-bottom -translate-x-1/2 -rotate-90 transform select-none overflow-hidden text-ellipsis whitespace-nowrap p-0 text-sm"
                        >
                            {{ dataStore.columns[ix].name }}
                        </div>
                    </div>
                </div>
            </div>

            <!-- Health Percentages -->
            <div class="mt-2 flex gap-2">
                <div
                    v-for="(colHealth, ix) in columnHealths"
                    :key="ix"
                    class="min-w-8 flex-grow select-none text-center text-sm"
                    :class="{
                        'font-semibold text-primary': colHealth < 1,
                    }"
                >
                    {{ formatPercentage(colHealth) }}
                </div>
            </div>
        </div>
    </div>
</template>
