<script setup lang="ts"></script>

<template>
    <div
        class="relative flex h-screen max-h-screen flex-col bg-pink-200/80 dark:bg-pink-800/80"
    >
        <div
            class="absolute inset-0 bg-pink-200/80 bg-[url('/abstract.svg')] bg-cover bg-center opacity-50 blur-md filter"
        ></div>

        <slot />
    </div>
</template>

<style scoped></style>
