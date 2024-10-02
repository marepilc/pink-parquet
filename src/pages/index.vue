<script setup lang="ts">
import useLoadConfig from '~/composables/use-load-config'

const route = useRoute()
const colorMode = useColorMode()
const config = ref<{ theme: 'light' | 'dark' } | null>(null)

onMounted(async () => {
    config.value = await useLoadConfig()
    colorMode.preference = config.value?.theme || 'light'
})
</script>

<template>
    <div>
        <div>{{ colorMode.value }}</div>
        <h1 class="font-semibold text-pink-500 dark:text-blue-500">
            Nuxt Routing set up successfully!
        </h1>
        <p>Current route: {{ route.path }}</p>
        <a href="https://nuxt.com/docs/getting-started/routing" target="_blank">
            Learn more about Nuxt Routing
        </a>
    </div>
</template>
