<script setup lang="ts">
import type { UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { extractPath } from '~/utils/path'

const colorMode = useColorMode()

const dataStore = useDataStore()
const tableStore = useTableStore()
const configStore = useConfigStore()

const unlisten = ref<UnlistenFn | null>(null)

onMounted(async () => {
    await configStore.loadConfig()
    colorMode.preference = configStore.theme

    unlisten.value = await getCurrentWebview().onDragDropEvent((event) => {
        if (event.payload.type === 'enter') {
            dataStore.updateDraggedFilePath(extractPath(event.payload.paths))
        } else if (event.payload.type === 'drop') {
            dataStore.updateOpenState(false, '', [0, 0])
            dataStore.resetContent()
            tableStore.deselectColumn()
            const extractedPath = extractPath(event.payload.paths)
            if (extractedPath) {
                dataStore.loadParquet(extractedPath)
            }
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

const uiFontClass = computed(() => {
    switch (configStore.uiFont) {
        case 'System UI':
            return 'font-system'
        case 'Humanist':
            return 'font-humanist'
        case 'Industrial':
            return 'font-industrial'
        default:
            return 'font-system'
    }
})
</script>
<template>
    <NuxtLayout>
        <div>
            <NuxtRouteAnnouncer />
        </div>
        <NuxtPage :class="uiFontClass" />
    </NuxtLayout>
</template>
