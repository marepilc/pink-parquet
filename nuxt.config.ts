export default defineNuxtConfig({
    // (optional) Enable the Nuxt devtools
    devtools: { enabled: true },

    // Enable SSG
    ssr: false,

    srcDir: 'src',
    css: ['~/assets/css/main.css', '~/assets/css/variables.css'],

    // Enables the development server to be discoverable by other devices when running on iOS physical devices
    devServer: { host: process.env.TAURI_DEV_HOST || 'localhost' },

    vite: {
        // Better support for Tauri CLI output
        clearScreen: false,
        // Enable environment variables
        // Additional environment variables can be found at
        // https://v2.tauri.app/reference/environment-variables/
        envPrefix: ['VITE_', 'TAURI_'],
        server: {
            // Tauri requires a consistent port
            strictPort: true,
        },
    },

    compatibilityDate: '2024-10-01',
    modules: [
        '@pinia/nuxt',
        // '@nuxt/ui',
        '@nuxtjs/tailwindcss',
        '@nuxtjs/color-mode',
        '@primevue/nuxt-module',
    ],
    primevue: {
        options: {
            ripple: true,
            tooltip: true,
            theme: 'none',
        },
    },
    postcss: {
        plugins: {
            'postcss-import': {},
            tailwindcss: {},
            autoprefixer: {},
        },
    },
})
