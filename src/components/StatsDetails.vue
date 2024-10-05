<script setup lang="ts">
// import { convertToDate, convertToDatetime } from '~/utils/format'

const tableStore = useTableStore()
const dataStore = useDataStore()

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
onMounted(() => {
    if (tableStore.selectedColumn) {
        console.log(dataStore.columnsInfo)
    }
})
</script>

<template>
    <div v-if="tableStore.selectedColumn">
        <div
            v-if="numericTypes.includes(tableStore.selectedColumn.dtype)"
            class="flex flex-col"
        >
            <div>
                min:
                {{
                    dataStore.columnsInfo[tableStore.selectedColumn.name]['min']
                }}
            </div>
            <div>
                max:
                {{
                    dataStore.columnsInfo[tableStore.selectedColumn.name]['max']
                }}
            </div>
            <div>
                mean:
                {{
                    dataStore.columnsInfo[tableStore.selectedColumn.name][
                        'mean'
                    ]
                }}
            </div>
            <div>
                25th percentile:
                {{
                    dataStore.columnsInfo[tableStore.selectedColumn.name][
                        'percentile_25'
                    ]
                }}
            </div>
            <div>
                median:
                {{
                    dataStore.columnsInfo[tableStore.selectedColumn.name][
                        'median'
                    ]
                }}
            </div>
            <div>
                75th percentile:
                {{
                    dataStore.columnsInfo[tableStore.selectedColumn.name][
                        'percentile_75'
                    ]
                }}
            </div>
            <div>
                nulls:
                {{
                    dataStore.columnsInfo[tableStore.selectedColumn.name][
                        'null_values'
                    ]
                }}
            </div>
        </div>
        <div v-else-if="tableStore.selectedColumn.dtype === 'Date'">
            <div>
                min:
                {{
                    convertToDate(
                        dataStore.columnsInfo[tableStore.selectedColumn.name][
                            'min'
                        ]
                    )
                }}
            </div>
            <div>
                max:
                {{
                    convertToDate(
                        dataStore.columnsInfo[tableStore.selectedColumn.name][
                            'max'
                        ]
                    )
                }}
            </div>
            <div>
                nulls:
                {{
                    dataStore.columnsInfo[tableStore.selectedColumn.name][
                        'null_values'
                    ]
                }}
            </div>
        </div>
        <div v-else-if="tableStore.selectedColumn.dtype === 'Datetime'">
            <div>
                min:
                {{
                    convertToDatetime(
                        dataStore.columnsInfo[tableStore.selectedColumn.name][
                            'min'
                        ]
                    )
                }}
            </div>
            <div>
                max:
                {{
                    convertToDatetime(
                        dataStore.columnsInfo[tableStore.selectedColumn.name][
                            'max'
                        ]
                    )
                }}
            </div>
            <div>
                nulls:
                {{
                    dataStore.columnsInfo[tableStore.selectedColumn.name][
                        'null_values'
                    ]
                }}
            </div>
        </div>
    </div>
</template>

<style scoped></style>
