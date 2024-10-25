<script setup lang="ts">
import {
    bytesForHuman,
    formatToLocalDateTime,
    removeExtension,
    numberThousandsSeparator,
} from '~/utils/format'
import ColumnInfo from '~/components/ColumnInfo.vue'
import TableInfo from '~/components/TableInfo.vue'

const dataStore = useDataStore()
const tableStore = useTableStore()

let createdDate = ''
let modifiedDate = ''

if (dataStore.fileMetadata) {
    createdDate = formatToLocalDateTime(dataStore.fileMetadata.createdAt)
    modifiedDate = formatToLocalDateTime(dataStore.fileMetadata.modifiedAt)
}
</script>

<template>
    <div v-if="dataStore.fileMetadata" class="flex min-h-52 w-full gap-2">
        <div class="flex min-w-40 max-w-64 flex-col items-center">
            <h2
                class="mb-3 w-full max-w-full overflow-hidden text-ellipsis whitespace-nowrap text-xl font-bold"
            >
                {{ removeExtension(dataStore.fileMetadata.fileName) }}
            </h2>

            <div class="flex flex-col">
                <div>
                    <span class="font-semibold">
                        {{ numberThousandsSeparator(dataStore.noOfColumns) }}
                    </span>
                    col. &#215;
                    <span class="font-semibold">
                        {{ numberThousandsSeparator(dataStore.noOfRows) }}
                    </span>
                    rows
                </div>
                <div v-if="dataStore.noOfRows !== dataStore.filteredRowsCount">
                    <span class="font-semibold">
                        {{
                            numberThousandsSeparator(
                                dataStore.filteredRowsCount
                            )
                        }}
                    </span>
                    rows filtered
                </div>
            </div>
            <div class="mt-2 flex flex-col items-center">
                <div class="text-right text-sm italic">Created:</div>
                <div class="text-sm font-semibold italic">
                    {{
                        formatToLocalDateTime(dataStore.fileMetadata.createdAt)
                    }}
                </div>
            </div>
            <div
                v-if="createdDate !== modifiedDate"
                class="mt-2 flex flex-col items-center"
            >
                <div class="text-right text-sm italic">Last modified:</div>
                <div class="text-sm font-semibold italic">
                    {{
                        formatToLocalDateTime(dataStore.fileMetadata.modifiedAt)
                    }}
                </div>
            </div>

            <div class="mt-2 flex flex-col items-center">
                <div class="text-right text-sm italic">File size:</div>
                <div class="text-sm font-semibold italic">
                    {{ bytesForHuman(dataStore.fileMetadata.fileSize) }}
                </div>
            </div>
        </div>
        <Divider layout="vertical" />
        <ColumnInfo v-if="tableStore.selectedColumn" />
        <TableInfo v-else />
    </div>
</template>

<style scoped></style>
