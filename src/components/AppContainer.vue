<script setup lang="ts">
const dataStore = useDataStore()
</script>

<template>
    <div class="relative flex h-full flex-col p-2">
        <div
            v-if="dataStore.isFileOpen && !dataStore.draggedFilePath"
            class="flex h-full flex-col gap-2"
        >
            <div class="frame flex min-h-0 grow flex-col">
                <slot name="table"></slot>
            </div>
            <div class="flex items-start gap-2">
                <div class="flex flex-col gap-2">
                    <div class="frame">
                        <div class="h-5 w-5">
                            <IconInfo />
                        </div>
                    </div>
                    <div class="frame">
                        <div class="h-5 w-5"></div>
                    </div>
                </div>

                <div class="frame grow">
                    <slot name="info"></slot>
                </div>
            </div>
            <div class="flex items-center gap-2">
                <div class="frame">
                    <button
                        type="button"
                        @click="$router.push('/settings')"
                        class="m-0 flex h-5 w-5 items-center justify-center border-none bg-transparent p-0 hover:text-pink-700"
                    >
                        <IconSettings class="h-5 w-5" />
                    </button>
                </div>
                <div class="frame min-w-0 grow">
                    <slot name="status"></slot>
                </div>
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
</template>

<style scoped></style>
