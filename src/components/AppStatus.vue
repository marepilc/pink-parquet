<script setup lang="ts">
const dataStore = useDataStore()

const closeFile = () => {
    dataStore.updateOpenState(false, '', [0, 0])
    dataStore.resetContent()
}
</script>

<template>
    <div class="flex w-full items-center justify-between">
        <div
            class="min-w-0 overflow-hidden text-ellipsis whitespace-nowrap pr-6 text-sm italic"
        >
            {{ dataStore.filePath }}
        </div>
        <button class="hover:text-pink-700" @click="closeFile()">
            <IconClose class="h-5 w-5" />
        </button>
    </div>
</template>
