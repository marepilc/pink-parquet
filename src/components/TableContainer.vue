<script setup lang="ts">
const dataStore = useDataStore()
</script>

<template>
    <div
        class="grid h-full w-full grid-cols-1 grid-rows-[1fr_256px_48px] items-center justify-stretch justify-items-center gap-2 overflow-hidden p-2"
    >
        <PpPanel>
            <slot name="table"></slot>
        </PpPanel>
        <PpPanel>
            <slot name="info"></slot>
        </PpPanel>
        <PpPanel class="flex w-full items-center justify-between gap-3">
            <Button
                class="h-8 w-8 p-1"
                type="button"
                @click="$router.push('/settings')"
            >
                <IconSettings class="h-5 w-5" />
            </Button>

            <slot name="status"></slot>
        </PpPanel>
    </div>
</template>

<style scoped></style>
