<script setup lang="ts">
const dataStore = useDataStore()
</script>

<template>
    <TableContainer v-if="dataStore.isFileOpen && !dataStore.draggedFilePath">
        <template v-slot:table>
            <AppTable />
        </template>
        <template v-slot:info>
            <AppInfo />
        </template>
        <template v-slot:status>
            <AppStatus />
        </template>
    </TableContainer>
    <FileLoader v-else />
</template>
