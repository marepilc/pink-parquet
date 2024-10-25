<script setup lang="ts"></script>

<template>
    <div
        class="relative flex h-screen max-h-screen flex-col bg-surface-200 dark:bg-surface-700"
    >
        <div
            class="absolute inset-0 bg-[url('/abstract.svg')] bg-cover bg-center opacity-10 blur-md filter"
        ></div>

        <slot />
    </div>
</template>

<style scoped></style>
