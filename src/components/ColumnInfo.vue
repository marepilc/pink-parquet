<script setup lang="ts">
import StatsDetails from '~/components/StatsDetails.vue'

const tableStore = useTableStore()
</script>

<template>
    <div class="flex grow flex-col border-l border-l-stone-500 px-2">
        <div class="flex flex-col">
            <div class="flex items-center gap-2">
                <DtypeIcon
                    class="h-5 w-5 flex-shrink-0"
                    :dtype="tableStore.selectedColumn.dtype"
                ></DtypeIcon>
                <h3 class="text-lg font-semibold">
                    {{ tableStore.selectedColumn.name }}
                </h3>
            </div>
            <StatsDetails v-if="tableStore.selectedColumn" />
        </div>
    </div>
</template>

<style scoped></style>
