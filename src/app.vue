<script setup lang="ts">
import type { UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { extractPath } from '~/utils/path'

const colorMode = useColorMode()

const dataStore = useDataStore()
const configStore = useConfigStore()

const unlisten = ref<UnlistenFn | null>(null)

onMounted(async () => {
    await configStore.loadConfig()
    colorMode.preference = configStore.theme

    unlisten.value = await getCurrentWebview().onDragDropEvent((event) => {
        if (event.payload.type === 'enter') {
            dataStore.updateDraggedFilePath(extractPath(event.payload.paths))
        } else if (event.payload.type === 'drop') {
            dataStore.loadParquet(extractPath(event.payload.paths))
        } else if (event.payload.type === 'leave') {
            dataStore.updateDraggedFilePath(null)
        }
    })
})

onUnmounted(() => {
    if (unlisten.value) {
        unlisten.value()
    }
})
</script>
<template>
    <NuxtLayout>
        <div>
            <NuxtRouteAnnouncer />
        </div>
        <NuxtPage />
    </NuxtLayout>
</template>
