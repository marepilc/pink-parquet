<script setup lang="ts">
import AppLogo from '~/components/AppLogo.vue'

const dataStore = useDataStore()
const infoVisible = ref(false)

function openInBrowser() {
    window.open('https://github.com/marepilc/pink-parquet', '_blank')
}
</script>

<template>
    <div
        class="grid h-full w-full grid-cols-1 grid-rows-[1fr_256px_48px] items-center justify-stretch justify-items-center gap-2 overflow-hidden p-2"
    >
        <Dialog
            v-model:visible="infoVisible"
            modal
            header="About Pink Parquet"
            :style="{ width: '30rem' }"
        >
            <template #container="{ closeCallback }">
                <div class="flex flex-col items-center space-y-4 p-6">
                    <p class="text-sm text-gray-500">Version: 1.0.0</p>
                    <AppLogo class="mb-4 w-48" />
                    <p class="text-center text-lg font-medium">
                        A user-friendly viewer for Parquet files.
                    </p>
                    <NuxtLink
                        to="https://github.com/marepilc/pink-parquet"
                        target="_blank"
                        class="text-primary underline hover:text-primary-600"
                    >
                        Visit the GitHub repository
                    </NuxtLink>
                    <Button
                        class="mt-6 w-24"
                        type="button"
                        label="OK"
                        @click="infoVisible = false"
                    />
                </div>
            </template>
        </Dialog>

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
            <Button
                class="h-8 w-8 p-1"
                type="button"
                @click="infoVisible = true"
            >
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
