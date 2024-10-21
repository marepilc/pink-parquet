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

const op = ref()
const columnFilterIx = ref(null)
const filterBtnClicked = (event, ix) => {
    console.log('filterBtnClicked', ix)
    console.log('event', event)
    op.value.hide()
    if (columnFilterIx.value === ix) {
        op.value.show(event)
    } else {
        columnFilterIx.value = ix
        nextTick(() => {
            op.value.show(event)
        })
    }
}
const clearFiltering = async () => {
    columnFilterIx.value = null
    tableStore.deselectColumn()
    // scroll to top
    const scrollable = document.querySelector('.scrollbar-custom')
    if (scrollable) {
        scrollable.scrollTo({ top: 0, behavior: 'instant' })
    }
    op.value.hide()
    await dataStore.loadParquet(dataStore.filePath)
}

const filterData = async () => {
    tableStore.deselectColumn()
    // scroll to top
    const scrollable = document.querySelector('.scrollbar-custom')
    if (scrollable) {
        scrollable.scrollTo({ top: 0, behavior: 'instant' })
    }
    op.value.hide()
    await dataStore.loadParquet(dataStore.filePath)
}

const speedDialItems = ref([
    {
        name: 'Clear Filters',
        icon: 'M14.8 11.975L6.825 4H19q.625 0 .9.55t-.1 1.05zm-.8 4.85V19q0 .425-.288.713T13 20h-2q-.425 0-.712-.288T10 19v-6.175l-7.9-7.9q-.275-.275-.275-.687t.275-.713q.3-.3.713-.3t.712.3L20.5 20.5q.3.3.288.7t-.313.7q-.3.275-.7.288t-.7-.288z',
        command: () => {
            tableStore.deselectColumn()
            // scroll to top
            const scrollable = document.querySelector('.scrollbar-custom')
            if (scrollable) {
                scrollable.scrollTo({ top: 0, behavior: 'instant' })
            }
            dataStore.clearFiltering()
            dataStore.loadParquet(dataStore.filePath)
        },
    },
    {
        name: 'Clear Sorting',
        icon: 'M24 22.303 22.273 24 11.606 13.333H2.105v-2.666h6.834l-4-4H2.105V4h.167L0 1.727 1.697 0 24 22.303ZM9.121 4h12.774v2.667H11.788L9.121 4ZM2.105 20v-2.667h6.597V20H2.105Z',
        command: () => {
            tableStore.deselectColumn()
            // scroll to top
            const scrollable = document.querySelector('.scrollbar-custom')
            if (scrollable) {
                scrollable.scrollTo({ top: 0, behavior: 'instant' })
            }
            dataStore.clearSorting()
            dataStore.loadParquet(dataStore.filePath)
        },
    },
])

function buttonEnabled(buttonName: string) {
    if (buttonName === 'Clear Filters') {
        return dataStore.filtering.length > 0
    } else if (buttonName === 'Clear Sorting') {
        return dataStore.sorting.length > 0
    }
}

const speedDialVisible = computed(() => {
    return dataStore.filtering.length > 0 || dataStore.sorting.length > 0
})
</script>

<template>
    <div class="relative h-full w-full">
        <SpeedDial
            v-if="speedDialVisible"
            :model="speedDialItems"
            :radius="80"
            direction="up"
            style="position: absolute; left: 10px; bottom: 16px; z-index: 9999"
        >
            <template #button="{ toggleCallback }">
                <Button class="m-0 h-8 w-8 p-1" @click="toggleCallback">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-6 w-6"
                        viewBox="0 0 24 24"
                    >
                        <path
                            fill="currentColor"
                            d="M19.775 22.625L15.05 17.9L13 20H4.75l-2.125-2.125q-.575-.575-.587-1.425T2.6 15l4.7-4.85l-5.925-5.925L2.8 2.8l18.4 18.4zM5.6 18h6.55l1.475-1.525L8.7 11.55L4 16.4zm12.275-2.975L16.45 13.6L20 9.95L15.05 5L11.5 8.65l-1.4-1.4l3.5-3.65q.575-.6 1.413-.6t1.412.575L21.4 8.55q.575.575.575 1.425T21.4 11.4zM11.175 14"
                        />
                    </svg>
                </Button>
            </template>
            <template #item="{ item, toggleCallback }">
                <Button
                    severity="secondary"
                    class="m-0 h-8 w-8 p-1 text-primary hover:!text-primary-700 disabled:text-surface-500 disabled:opacity-100 hover:disabled:!text-surface-500"
                    @click="toggleCallback"
                    raised
                    :disabled="!buttonEnabled(item.name)"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-6 w-6"
                        viewBox="0 0 24 24"
                    >
                        <path fill="currentColor" :d="item.icon" />
                    </svg>
                </Button>
            </template>
        </SpeedDial>
        <div
            class="scrollbar-custom h-full w-full overflow-auto"
            @scroll="onScroll"
        >
            <Popover ref="op">
                <div
                    v-if="columnFilterIx !== null"
                    class="flex items-center gap-2"
                >
                    <FilterPanel
                        :colIx="columnFilterIx"
                        @clear="clearFiltering"
                        @filter="filterData"
                    />
                </div>
            </Popover>
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
                                        <Button
                                            class="h-4 w-4 flex-shrink-0"
                                            @click.stop="
                                                filterBtnClicked($event, ixCol)
                                            "
                                            :class="
                                                dataStore.columnInFiltering(
                                                    column.name
                                                )
                                                    ? 'text-primary'
                                                    : 'text-surface-500'
                                            "
                                            :disabled="
                                                tableStore.filterDisabled(
                                                    column
                                                )
                                            "
                                            text
                                        >
                                            <template #icon>
                                                <svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    viewBox="0 0 24 24"
                                                    class="h-3 w-3 flex-shrink-0"
                                                >
                                                    <path
                                                        fill="currentColor"
                                                        d="M11 20q-.425 0-.712-.288T10 19v-6L4.2 5.6q-.375-.5-.112-1.05T5 4h14q.65 0 .913.55T19.8 5.6L14 13v6q0 .425-.288.713T13 20z"
                                                    />
                                                </svg>
                                            </template>
                                        </Button>

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
                                <div class="flex flex-col">
                                    <Button
                                        @click.stop="
                                            handleSorting({
                                                column: column.name,
                                                ascending: true,
                                            })
                                        "
                                        class="h-4 w-4 text-surface-500"
                                        :class="{
                                            '!text-primary':
                                                dataStore.indexInSorting({
                                                    column: column.name,
                                                    ascending: true,
                                                }),
                                        }"
                                        text
                                    >
                                        <template #icon>
                                            <svg
                                                viewBox="0 0 24 24"
                                                class="h-3 w-3"
                                                xmlns="http://www.w3.org/2000/svg"
                                                fill="currentColor"
                                            >
                                                <path
                                                    d="m12 3.167 10.406 17.666H1.595L12 3.167Z"
                                                />
                                            </svg>
                                        </template>
                                    </Button>
                                    <Button
                                        @click.stop="
                                            handleSorting({
                                                column: column.name,
                                                ascending: false,
                                            })
                                        "
                                        class="h-4 w-4 text-surface-500"
                                        :class="{
                                            '!text-primary':
                                                dataStore.indexInSorting({
                                                    column: column.name,
                                                    ascending: false,
                                                }),
                                        }"
                                        text
                                    >
                                        <template #icon>
                                            <svg
                                                viewBox="0 0 24 24"
                                                class="h-3 w-3"
                                                xmlns="http://www.w3.org/2000/svg"
                                                fill="currentColor"
                                            >
                                                <path
                                                    d="M12 20.833 1.594 3.167h21.812L12 20.833Z"
                                                />
                                            </svg>
                                        </template>
                                    </Button>

                                    <div
                                        class="flex h-4 items-center justify-center text-primary"
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

                <tbody
                    :class="tableFontClass"
                    :key="dataStore.rowsLoadingCounter"
                >
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
