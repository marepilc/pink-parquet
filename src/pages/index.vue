<script setup lang="ts">
const route = useRoute()
</script>

<template>
    <AppContainer>
        <template v-slot:table>
            <div>table</div>
        </template>
        <template v-slot:info>
            <div class="h-44">info</div>
        </template>
        <template v-slot:status>
            <div>
                <NuxtLink to="/settings">Settings</NuxtLink>
            </div>
        </template>
    </AppContainer>
</template>
