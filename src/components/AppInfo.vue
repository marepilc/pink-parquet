<script setup lang="ts">
import { bytesForHuman, formatToLocalDateTime } from '~/utils/format'

const dataStore = useDataStore()
</script>

<template>
    <div class="flex min-h-44 flex-col">
        <h2
            class="overflow-hidden text-ellipsis whitespace-nowrap text-xl font-bold"
        >
            {{ dataStore.fileMetadata.fileName }}
        </h2>

        <div>
            <span class="font-semibold">{{ dataStore.noOfColumns }}</span>
            columns
        </div>
        <div>
            <span class="font-semibold">{{ dataStore.noOfRows }}</span>
            rows
        </div>
        <div class="flex">
            <div class="flex flex-col">
                <div class="mt-2 pr-2 text-right text-sm italic">Created:</div>
                <div class="pr-2 text-right text-sm italic">Last modified:</div>
                <div class="pr-2 text-right text-sm italic">File size:</div>
            </div>
            <div class="flex flex-col">
                <div class="mt-2 text-sm font-semibold italic">
                    {{
                        formatToLocalDateTime(dataStore.fileMetadata.createdAt)
                    }}
                </div>
                <div class="text-sm font-semibold italic">
                    {{
                        formatToLocalDateTime(dataStore.fileMetadata.modifiedAt)
                    }}
                </div>
                <div class="text-sm font-semibold italic">
                    {{ bytesForHuman(dataStore.fileMetadata.size) }}
                </div>
            </div>
        </div>
        <div class="mt-2 text-sm italic">
            {{ dataStore.rows.length }} rows loaded for preview
        </div>
    </div>
</template>

<style scoped></style>
