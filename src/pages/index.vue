<script setup lang="ts">
const route = useRoute()
</script>

<template>
    <AppContainer>
        <template v-slot:table>
            <div class="font-mono">table EUR Europa 1 | 13:40</div>
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
