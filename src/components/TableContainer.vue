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
        <PpPanel
            class="scrollbar-custom flex w-full items-center overflow-x-auto"
        >
            <slot name="info"></slot>
        </PpPanel>
        <div class="flex w-full items-center gap-2">
            <Button
                class="h-8 w-8 p-1"
                type="button"
                @click="$router.push('/settings')"
            >
                <IconSettings class="h-5 w-5" />
            </Button>
            <Button class="h-8 w-8 p-1" type="button">
                <IconAbout class="h-5 w-5" />
            </Button>
            <PpPanel
                class="flex w-auto flex-grow items-center justify-between gap-3"
            >
                <slot name="status"></slot>
            </PpPanel>
        </div>
    </div>
</template>

<style scoped></style>
