<script setup lang="ts">
const colorMode = useColorMode()
const config = ref<{ theme: 'light' | 'dark' } | null>(null)

onMounted(async () => {
    config.value = await useLoadConfig()
    colorMode.preference = config.value?.theme || 'light'
})
</script>
<template>
    <div>
        <NuxtRouteAnnouncer />
    </div>
    <NuxtPage />
</template>
