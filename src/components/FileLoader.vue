<script setup lang="ts">
const dataStore = useDataStore()
</script>

<template>
    <div
        class="flex h-full min-h-0 w-full min-w-0 flex-col items-center justify-center gap-4 overflow-hidden rounded-2xl border-2 border-dashed border-pink-700 bg-stone-50/75 dark:bg-stone-900/90"
    >
        <div
            class="h-32 w-32 text-pink-700"
            :class="{ 'animate-bounce': dataStore.draggedFilePath }"
        >
            <IconParquetFile />
        </div>
        <div
            v-if="dataStore.draggedFilePath"
            class="grow-1 z-10 mx-auto flex w-[90%] justify-center overflow-hidden break-words rounded-md border border-surface-400 bg-surface-100 px-4 py-2 text-center dark:border-surface-600 dark:bg-surface-800"
        >
            {{ dataStore.draggedFilePath }}
        </div>
        <div class="text-4xl font-bold text-primary-700">DROP ZONE</div>
    </div>
</template>

<style scoped>
/* Ensure long words are broken to fit within the container */
.flex > .grow-1 {
    word-break: break-all;
}
</style>
