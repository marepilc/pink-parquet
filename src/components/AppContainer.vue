<script setup lang="ts">
const dataStore = useDataStore()
</script>

<template>
    <!-- Background Layer (always present) -->
    <div
        class="relative flex h-screen max-h-screen flex-col bg-pink-200/80 dark:bg-pink-800/80"
    >
        <!-- Shared Background Layer -->
        <div
            class="absolute inset-0 bg-pink-200/80 bg-[url('/abstract.svg')] bg-cover bg-center opacity-50 blur-md filter"
        ></div>

        <!-- Conditional Content (v-if/v-else) -->
        <div class="relative flex h-full flex-col p-2">
            <div
                v-if="dataStore.isFileOpen && !dataStore.draggedFilePath"
                class="flex h-full flex-col gap-2"
            >
                <div class="frame flex min-h-0 grow flex-col">
                    <slot name="table"></slot>
                </div>
                <div class="frame">
                    <slot name="info"></slot>
                </div>
                <div class="frame">
                    <slot name="status"></slot>
                </div>
            </div>

            <!-- v-else Block -->
            <div v-else class="flex h-full items-center justify-center">
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
            </div>
        </div>
    </div>
</template>

<style scoped>
.frame {
    @apply rounded-md border border-stone-700 bg-stone-50/90 p-2 dark:border-pink-800 dark:bg-stone-950/90;
}
</style>
