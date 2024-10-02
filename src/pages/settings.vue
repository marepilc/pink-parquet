<script setup lang="ts">
const configStore = useConfigStore()
const colorMode = useColorMode()

function toggleTheme() {
    configStore.toggleTheme()
    colorMode.preference = configStore.theme
}
</script>

<template>
    <div>settings</div>
    <button @click="toggleTheme">
        Toggle Color Mode ({{ colorMode.preference }})
    </button>
    <br />
    <br />
    <br />
    <NuxtLink class="mt-4 border border-pink-700 p-2" to="/">App</NuxtLink>
</template>

<style scoped></style>
