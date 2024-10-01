<script setup lang="ts">
const route = useRoute()
const dataStore = useDataStore()
</script>

<template>
  <div>
    <h1 class="font-semibold">Nuxt Routing set up successfully!</h1>
    <p>Current route: {{ route.path }}</p>
    <a href="https://nuxt.com/docs/getting-started/routing" target="_blank">Learn more about Nuxt Routing</a>
  </div>
</template>
