<script setup lang="ts">
import type { Sorting } from '~/types/app-types'
import FilterPanel from '~/components/FilterPanel.vue'

const dataStore = useDataStore()
const tableStore = useTableStore()
const configStore = useConfigStore()

const columnWidths = ref<number[]>([])

const totalTableWidth = computed(() => {
    return columnWidths.value.reduce((acc, width) => acc + width, 0)
})

onMounted(() => {
    columnWidths.value = dataStore.columns.map(() => 150)
})

let isResizing = ref(false)
let currentColIndex = ref<number | null>(null)
let startX = ref<number>(0)
let startWidth = ref<number>(0)

function onMouseDown(e: MouseEvent, ixCol: number) {
    isResizing.value = true
    currentColIndex.value = ixCol
    startX.value = e.clientX
    startWidth.value = columnWidths.value[ixCol]

    document.addEventListener('mousemove', onMouseMove)
    document.addEventListener('mouseup', onMouseUp)
}

function onMouseMove(e: MouseEvent) {
    if (!isResizing.value || currentColIndex.value === null) return

    const deltaX = e.clientX - startX.value
    const newWidth = startWidth.value + deltaX
    if (newWidth > 100) {
        // Minimum width
        columnWidths.value[currentColIndex.value] = newWidth
    }
}

function onMouseUp() {
    isResizing.value = false
    currentColIndex.value = null

    document.removeEventListener('mousemove', onMouseMove)
    document.removeEventListener('mouseup', onMouseUp)
}

async function onScroll(e: Event) {
    const target = e.target as HTMLElement
    if (target.scrollHeight - target.scrollTop === target.clientHeight) {
        if (!dataStore.allRowsLoaded) {
            await dataStore.getMoreRows()
        }
    }
}

async function handleSorting(sorting: Sorting) {
    tableStore.deselectColumn()
    // scroll to top
    const scrollable = document.querySelector('.scrollbar-custom')
    if (scrollable) {
        scrollable.scrollTo({ top: 0, behavior: 'instant' })
    }
    dataStore.updateSorting(sorting)
    await dataStore.loadParquet(dataStore.filePath)
}

async function handleFiltering() {
    // tableStore.deselectColumn()
    // scroll to top
    // const scrollable = document.querySelector('.scrollbar-custom')
}

function selectColumn(ixCol: number) {
    tableStore.selectColumn(ixCol)
}

const tableFontClass = computed(() => {
    switch (configStore.tableFont) {
        case 'Ubuntu Mono':
            return 'font-ubuntu'
        case 'Departure Mono':
            return 'font-departure'
        default:
            return 'font-ubuntu'
    }
})

const filterPanelOpen = ref(false)
</script>

<template>
    <div
        class="scrollbar-custom relative h-full overflow-auto"
        @scroll="onScroll"
    >
        <table
            class="w-full table-fixed cursor-default text-left text-stone-950 dark:text-stone-50"
            :style="{ width: `${totalTableWidth}px` }"
        >
            <!-- Colgroup for column definitions -->
            <colgroup>
                <col
                    v-for="(column, ixCol) in dataStore.columns"
                    :key="ixCol"
                    :class="{
                        'bg-pink-300': tableStore.selectedColumn === column,
                    }"
                    :style="{ width: `${columnWidths[ixCol]}px` }"
                />
            </colgroup>

            <thead
                class="sticky-header sticky top-0 z-10 bg-stone-200 dark:bg-stone-800"
            >
                <tr>
                    <th
                        v-for="(column, ixCol) in dataStore.columns"
                        :key="ixCol"
                        class="relative max-w-64 overflow-hidden text-ellipsis whitespace-nowrap border-r border-stone-300 px-3 py-1.5 dark:border-stone-600"
                        :class="{
                            'bg-pink-300 dark:bg-pink-950':
                                tableStore.selectedColumn === column,
                        }"
                        scope="col"
                    >
                        <div
                            class="flex justify-between gap-2"
                            @click="selectColumn(ixCol)"
                        >
                            <div class="flex min-w-8 flex-col">
                                <div class="flex items-center gap-1">
                                    <UPopover @click.stop>
                                        <UButton
                                            size="2xs"
                                            class="text-stone-400 hover:text-pink-700"
                                            square
                                            icon="material-symbols:filter-alt"
                                            variant="link"
                                            @click="handleFiltering"
                                        />
                                        <template #panel>
                                            <div class="p-4">
                                                <div class="min-h-20">
                                                    <FilterPanel
                                                        :columnName="
                                                            column.name
                                                        "
                                                        @close="
                                                            filterPanelOpen = false
                                                        "
                                                        :dtype="column.dtype"
                                                    />
                                                </div>
                                            </div>
                                        </template>
                                    </UPopover>

                                    <div
                                        class="overflow-hidden text-ellipsis whitespace-nowrap"
                                    >
                                        {{ column.name }}
                                    </div>
                                </div>
                                <div
                                    class="flex items-center gap-2 text-xs text-stone-700 dark:text-stone-400"
                                >
                                    <DtypeIcon
                                        class="h-5 w-5 flex-shrink-0 text-pink-700"
                                        :dtype="column.dtype"
                                    ></DtypeIcon>
                                    <div
                                        class="overflow-hidden text-ellipsis whitespace-nowrap"
                                    >
                                        {{ column.dtype }}
                                    </div>
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
                        <div
                            class="absolute right-0 top-0 h-full w-1 cursor-col-resize bg-transparent"
                            @mousedown.prevent="onMouseDown($event, ixCol)"
                        ></div>
                    </th>
                </tr>
            </thead>

            <tbody :class="tableFontClass" :key="dataStore.rowsLoadingCounter">
                <tr
                    v-for="(row, ixRow) in dataStore.rows"
                    :key="ixRow"
                    class="whitespace-nowrap px-2 py-1 odd:bg-stone-50/65 even:bg-stone-200/65 dark:odd:bg-stone-800/65 dark:even:bg-stone-600/65"
                >
                    <td
                        v-for="(cell, ixCell) in row"
                        :key="ixCell"
                        class="max-w-40 overflow-hidden text-ellipsis border-b border-r border-stone-300 px-3 py-1.5 dark:border-stone-600"
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
