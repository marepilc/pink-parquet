<script setup lang="ts">
import AppStatus from '~/components/AppStatus.vue'
</script>

<template>
    <AppContainer>
        <template v-slot:table>
            <AppTable />
        </template>
        <template v-slot:info>
            <div class="h-44">info</div>
        </template>
        <template v-slot:status>
            <AppStatus />
        </template>
    </AppContainer>
</template>
