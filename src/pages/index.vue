<script setup lang="ts">
const route = useRoute()
const dataStore = useDataStore()
const colorMode = useColorMode()
</script>

<template>
  <div>
    <button
    @click="colorMode.preference = colorMode.preference === 'light' ? 'dark' : 'light'"
    >Change color mode</button>
    <div>{{colorMode.value}}</div>
    <h1 class="font-semibold text-pink-500 dark:text-blue-500">Nuxt Routing set up successfully!</h1>
    <p>Current route: {{ route.path }}</p>
    <a href="https://nuxt.com/docs/getting-started/routing" target="_blank">Learn more about Nuxt Routing</a>
  </div>
</template>
