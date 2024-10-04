<script setup lang="ts">
import type { Sorting } from '~/types/app-types'

const dataStore = useDataStore()
const tableStore = useTableStore()

async function onScroll(e: Event) {
    const target = e.target as HTMLElement
    if (target.scrollHeight - target.scrollTop === target.clientHeight) {
        if (!dataStore.allRowsLoaded) {
            await dataStore.getMoreRows()
        }
    }
}

async function handleSorting(sorting: Sorting) {
    // scroll to top
    const scrollable = document.querySelector('.scrollbar-custom')
    if (scrollable) {
        scrollable.scrollTo({ top: 0, behavior: 'instant' })
    }
    dataStore.updateSorting(sorting)
    if (dataStore.sorting.length > 0) {
        await dataStore.loadParquet(dataStore.filePath)
    }
}

function selectColumn(ixCol: number) {
    tableStore.selectColumn(ixCol)
}
</script>

<template>
    <div
        class="scrollbar-custom relative h-full overflow-auto"
        @scroll="onScroll"
    >
        <table class="w-full text-left text-stone-950">
            <!-- Colgroup for column definitions -->
            <colgroup>
                <col
                    v-for="(column, ixCol) in dataStore.columns"
                    :key="ixCol"
                    :class="{
                        'bg-pink-300': tableStore.selectedColumn === column,
                    }"
                />
            </colgroup>

            <thead class="sticky-header sticky top-0 z-10 bg-stone-200">
                <tr>
                    <th
                        v-for="(column, ixCol) in dataStore.columns"
                        :key="ixCol"
                        class="max-w-40 overflow-hidden text-ellipsis whitespace-nowrap border-r border-stone-300 bg-stone-200 px-3 py-1.5"
                        scope="col"
                    >
                        <div
                            class="flex justify-between gap-2"
                            @click="selectColumn(ixCol)"
                        >
                            <div class="flex flex-col">
                                <div>{{ column.name }}</div>
                                <div
                                    class="flex items-center gap-2 text-xs text-stone-700"
                                >
                                    <DtypeIcon
                                        class="h-5 w-5 text-pink-700"
                                        :dtype="column.dtype"
                                    ></DtypeIcon>
                                    <div>{{ column.dtype }}</div>
                                </div>
                            </div>
                            <div class="flex flex-col text-stone-400">
                                <ActionBtn
                                    @click.stop="
                                        handleSorting({
                                            column: column.name,
                                            ascending: true,
                                        })
                                    "
                                    class="h-4 w-4 hover:text-pink-700"
                                    :class="{
                                        'text-pink-700':
                                            dataStore.indexInSorting({
                                                column: column.name,
                                                ascending: true,
                                            }),
                                    }"
                                >
                                    <IconAsc />
                                </ActionBtn>
                                <ActionBtn
                                    @click.stop="
                                        handleSorting({
                                            column: column.name,
                                            ascending: false,
                                        })
                                    "
                                    class="h-4 w-4 hover:text-pink-700"
                                    :class="{
                                        'text-pink-700':
                                            dataStore.indexInSorting({
                                                column: column.name,
                                                ascending: false,
                                            }),
                                    }"
                                >
                                    <IconDesc />
                                </ActionBtn>
                                <div
                                    class="flex h-4 items-center justify-center text-pink-700"
                                >
                                    {{
                                        dataStore.columnIndexInSorting(
                                            column.name
                                        )
                                    }}
                                </div>
                            </div>
                        </div>
                    </th>
                </tr>
            </thead>

            <tbody class="font-mono" :key="dataStore.rowsLoadingCounter">
                <tr
                    v-for="(row, ixRow) in dataStore.rows"
                    :key="ixRow"
                    class="whitespace-nowrap px-2 py-1 odd:bg-stone-50/65 even:bg-stone-200/65"
                >
                    <td
                        v-for="(cell, ixCell) in row"
                        :key="ixCell"
                        class="max-w-40 overflow-hidden text-ellipsis border-b border-r border-stone-300 px-3 py-1.5"
                    >
                        {{ cell }}
                    </td>
                </tr>
            </tbody>
        </table>
    </div>
</template>

<style scoped>
.sticky-header::before,
.sticky-header::after {
    content: '';
    position: absolute;
    left: 0;
    right: 0;
    height: 1px;
    background-color: rgba(219, 39, 119, 1); /* Adjust the color as needed */
    pointer-events: none; /* Allows clicks to pass through */
}

.sticky-header::before {
    top: 0;
}

.sticky-header::after {
    bottom: 0;
}
</style>
