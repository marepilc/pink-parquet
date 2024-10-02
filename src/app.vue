<script setup lang="ts">
import type { UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWebview } from '@tauri-apps/api/webview'

const colorMode = useColorMode()
const config = ref<{ theme: 'light' | 'dark' } | null>(null)

const unlisten = ref<UnlistenFn | null>(null)

onMounted(async () => {
    config.value = await useLoadConfig()
    colorMode.preference = config.value?.theme || 'light'

    unlisten.value = await getCurrentWebview().onDragDropEvent((event) => {
        if (event.payload.type === 'enter') {
            console.log('enter', event.payload)
        } else if (event.payload.type === 'drop') {
            console.log('drop', event.payload)
        } else if (event.payload.type === 'leave') {
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
    <div>
        <NuxtRouteAnnouncer />
    </div>
    <NuxtPage />
</template>
