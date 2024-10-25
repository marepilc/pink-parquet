<script setup lang="ts">
import StatsDetails from '~/components/StatsDetails.vue'

const tableStore = useTableStore()
</script>

<template>
    <div class="flex grow flex-col px-2" v-if="tableStore.selectedColumn">
        <div class="flex flex-col">
            <div class="mb-2 flex items-center justify-between gap-2">
                <div class="flex items-center gap-2">
                    <DtypeIcon
                        class="h-6 w-6 flex-shrink-0 text-primary"
                        :dtype="tableStore.selectedColumn.dtype"
                    ></DtypeIcon>
                    <h3 class="text-lg font-semibold">
                        {{ tableStore.selectedColumn.name }}
                    </h3>
                </div>
                <div class="flex items-center gap-2">
                    <IconCompress class="h-6 w-6" />
                    <span class="text-sm">
                        {{ tableStore.selectedColumn.compression }}
                    </span>
                </div>
            </div>
            <StatsDetails :dtype="tableStore.selectedColumn.dtype" />
        </div>
    </div>
</template>

<style scoped></style>
