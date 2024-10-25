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
                <Button
                    size="sm"
                    color="primary"
                    square
                    variant="solid"
                    @click="toggleTheme"
                >
                    <IconTheme class="h-6 w-6" />
                </Button>
                <label class="ml-2 text-sm">{{ configStore.theme }}</label>
            </div>
            <div class="flex flex-col gap-1">
                <label class="text-sm" for="ui-font-menu">UI Font</label>
                <Select
                    v-model="uiFontSelected"
                    :options="uiFont"
                    @change="setUIFont"
                />
            </div>
            <div class="flex flex-col gap-1">
                <label class="text-sm" for="table-font-menu">Table Font</label>
                <Select
                    id="table-font-menu"
                    v-model="tableFontSelected"
                    :options="tableFont"
                    @change="setTableFont"
                />
            </div>
            <Button
                class="mb-4 self-center"
                @click="$router.push('/')"
                label="Close"
            >
                <template v-slot:icon>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        class="h-5 w-5"
                    >
                        <path
                            fill="currentColor"
                            d="M10.925 14.05L16.6 8.4l-1.425-1.425l-4.25 4.25L8.8 9.1l-1.4 1.4zM1 21v-2h22v2zm3-3q-.825 0-1.412-.587T2 16V5q0-.825.588-1.412T4 3h16q.825 0 1.413.588T22 5v11q0 .825-.587 1.413T20 18zm0-2h16V5H4zm0 0V5z"
                        />
                    </svg>
                </template>
            </Button>
        </div>
    </div>
</template>

<style scoped></style>
