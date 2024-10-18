<script setup lang="ts">
const dataStore = useDataStore()
</script>

<template>
    <div
        class="flex h-full w-full flex-col items-center justify-center gap-4 rounded-2xl border-2 border-dashed border-pink-700 bg-stone-50/75 dark:bg-stone-900/90"
    >
        <div
            class="h-32 w-32 text-pink-700"
            :class="{ 'animate-bounce': dataStore.draggedFilePath }"
        >
            <IconParquetFile />
        </div>
        <p>{{ dataStore.draggedFilePath }}</p>
    </div>
</template>

<style scoped></style>
