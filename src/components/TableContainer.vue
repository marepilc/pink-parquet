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
        <PpPanel>
            <div class="frame">
                <button
                    type="button"
                    @click="$router.push('/settings')"
                    class="m-0 flex h-5 w-5 items-center justify-center border-none bg-transparent p-0 hover:text-pink-700"
                >
                    <IconSettings class="h-5 w-5" />
                </button>
            </div>
            <slot name="status"></slot>
        </PpPanel>
    </div>
</template>

<style scoped></style>
