<script setup lang="ts">
import { bytesForHuman, formatToLocalDateTime } from '~/utils/format'

const dataStore = useDataStore()
</script>

<template>
    <div class="flex min-h-44 flex-col">
        <h2 class="text-xl font-bold">{{ dataStore.fileMetadata.fileName }}</h2>
        <div>
            <span class="font-semibold">{{ dataStore.noOfColumns }}</span>
            columns
        </div>
        <div>
            <span class="font-semibold">{{ dataStore.noOfRows }}</span>
            rows
        </div>
        <div class="mt-2 text-sm italic">
            Created:
            {{ formatToLocalDateTime(dataStore.fileMetadata.createdAt) }}
        </div>
        <div class="text-sm italic">
            Last modified:
            {{ formatToLocalDateTime(dataStore.fileMetadata.modifiedAt) }}
        </div>
        <div class="text-sm italic">
            File zize
            <span class="font-semibold">
                {{ bytesForHuman(dataStore.fileMetadata.size) }}
            </span>
        </div>
        <div class="mt-2 text-sm italic">
            {{ dataStore.rows.length }} rows loaded for preview
        </div>
    </div>
</template>

<style scoped></style>
