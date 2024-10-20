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

const createdDate = formatToLocalDateTime(dataStore.fileMetadata.createdAt)
const modifiedDate = formatToLocalDateTime(dataStore.fileMetadata.modifiedAt)
</script>

<template>
    <div class="flex min-h-52 w-full gap-2">
        <div class="flex min-w-40 max-w-64 flex-col items-center">
            <h2
                class="mb-4 overflow-hidden text-ellipsis whitespace-nowrap text-xl font-bold"
            >
                {{ removeExtension(dataStore.fileMetadata.fileName) }}
            </h2>

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
            <div class="mt-2 flex flex-col items-center gap-0.5">
                <div class="text-right text-sm italic">Created:</div>
                <div class="text-sm font-semibold italic">
                    {{
                        formatToLocalDateTime(dataStore.fileMetadata.createdAt)
                    }}
                </div>
            </div>
            <div
                v-if="createdDate !== modifiedDate"
                class="mt-2 flex flex-col items-center gap-0.5"
            >
                <div class="text-right text-sm italic">Last modified:</div>
                <div class="text-sm font-semibold italic">
                    {{
                        formatToLocalDateTime(dataStore.fileMetadata.modifiedAt)
                    }}
                </div>
            </div>

            <div class="mt-2 flex flex-col items-center gap-0.5">
                <div class="text-right text-sm italic">File size:</div>
                <div class="text-sm font-semibold italic">
                    {{ bytesForHuman(dataStore.fileMetadata.size) }}
                </div>
            </div>
        </div>
        <ColumnInfo v-if="tableStore.selectedColumn" />
        <TableInfo v-else />
    </div>
</template>

<style scoped></style>
