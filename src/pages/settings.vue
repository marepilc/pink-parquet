<script setup lang="ts">
const configStore = useConfigStore()
const colorMode = useColorMode()

const darkModeOn = ref<boolean>(false)

watch(
    () => configStore.theme,
    (theme) => {
        darkModeOn.value = theme === 'dark'
    },
    { immediate: true }
)

onMounted(() => {
    nextTick(() => {
        darkModeOn.value = configStore.theme === 'dark'
    })
})

function toggleTheme() {
    configStore.toggleTheme()
    setTimeout(() => {
        colorMode.preference = configStore.theme
    }, 500)
}
</script>

<template>
    <div
        class="frame relative z-10 m-2 flex h-full items-center justify-center p-2"
    >
        <div
            class="flex flex-col rounded-md border border-stone-500 bg-stone-50 p-4 dark:border-stone-700 dark:bg-stone-950"
        >
            <h2 class="mb-4 text-center text-xl font-bold">settings</h2>
            <AppSwitch
                class="mb-4"
                v-model="darkModeOn"
                label="Dark Mode"
                @change="toggleTheme"
            />
            <button class="btn-primary" @click="$router.push('/')">
                Close
            </button>
        </div>
    </div>
</template>

<style scoped></style>
