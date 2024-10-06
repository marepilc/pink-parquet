<script setup lang="ts">
const configStore = useConfigStore()
const colorMode = useColorMode()

const theme = ref<string>(null)

const uiFont = ['System UI', 'Humanist', 'Industrial']
const uiFontSelected = ref<string>(uiFont[0])

const tableFont = ['Ubuntu Mono', 'Departure Mono']
const tableFontSelected = ref<string>(tableFont[0])

onMounted(() => {
    nextTick(() => {
        theme.value = configStore.theme
        uiFontSelected.value = configStore.uiFont
        tableFontSelected.value = configStore.tableFont
    })
})

function toggleTheme() {
    configStore.toggleTheme()
    setTimeout(() => {
        colorMode.preference = configStore.theme
    }, 250)
}

function setUIFont() {
    configStore.setUIFont(uiFontSelected.value)
}

function setTableFont() {
    configStore.setTableFont(tableFontSelected.value)
}
</script>

<template>
    <div
        class="frame relative z-10 m-2 flex h-full items-center justify-center p-2"
    >
        <div
            class="flex min-w-60 flex-col gap-4 rounded-md border border-stone-500 bg-stone-50 p-4 dark:border-stone-700 dark:bg-stone-950"
        >
            <h2 class="mb-4 text-center text-xl font-bold">settings</h2>
            <div class="flex items-center gap-2">
                <UButton
                    size="sm"
                    color="primary"
                    square
                    variant="solid"
                    @click="toggleTheme"
                >
                    <IconTheme class="h-6 w-6" />
                </UButton>
                <label class="ml-2 text-sm">{{ configStore.theme }}</label>
            </div>
            <div>
                <label class="text-sm" for="ui-font-menu">UI Font</label>
                <USelectMenu
                    id="ui-font-menu"
                    v-model="uiFontSelected"
                    :options="uiFont"
                    @change="setUIFont"
                />
            </div>
            <div>
                <label class="text-sm" for="table-font-menu">Table Font</label>
                <USelectMenu
                    id="table-font-menu"
                    v-model="tableFontSelected"
                    :options="tableFont"
                    @change="setTableFont"
                />
            </div>
            <UButton
                class="mb-4 self-center"
                @click="$router.push('/')"
                variant="solid"
                icon="material-symbols:sync-saved-locally-outline"
                label="Close"
            ></UButton>
        </div>
    </div>
</template>

<style scoped></style>
