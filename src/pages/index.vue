<script setup lang="ts">
import AppStatus from '~/components/AppStatus.vue'
import AppInfo from '~/components/AppInfo.vue'
</script>

<template>
    <AppContainer>
        <template v-slot:table>
            <AppTable />
        </template>
        <template v-slot:info>
            <AppInfo />
        </template>
        <template v-slot:status>
            <AppStatus />
        </template>
    </AppContainer>
</template>
