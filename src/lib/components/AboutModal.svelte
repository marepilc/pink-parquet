<script lang="ts">
    import AppLogo from '$lib/components/AppLogo.svelte'
    import {dataStore} from '$lib/stores/dataStore.svelte'

    const currentYear = new Date().getFullYear()
    const copyrightYear = currentYear > 2024 ? `2024–${currentYear}` : '2024'

    let {isOpen = $bindable(false)} = $props<{ isOpen?: boolean }>()

    $effect(() => {
        if (isOpen && dataStore.updateCount > 0) {
            dataStore.updateSeen = true
        }
    })

    function closeModal() {
        isOpen = false
    }

    function handleBackdropClick(event: MouseEvent) {
        if (event.target === event.currentTarget) {
            closeModal()
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape') {
            closeModal()
        }
    }

    async function openLink(url: string) {
        try {
            // Use Tauri's opener plugin via invoke
            const {invoke} = await import('@tauri-apps/api/core')
            await invoke('plugin:opener|open_url', {url})
        } catch (error) {
            console.error('Failed to open link:', error)
        }
    }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
            class="modal-backdrop"
            role="dialog"
            aria-modal="true"
            aria-labelledby="about-title"
            onclick={handleBackdropClick}
            onkeydown={handleKeydown}
            tabindex="0"
    >
        <div class="modal-content">
            <div class="modal-header">
                <h2 id="about-title">About</h2>
                <button class="close-button" onclick={closeModal} aria-label="Close">
                    <svg
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                                d="M18 6L6 18M6 6L18 18"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                        />
                    </svg>
                </button>
            </div>

            <div class="modal-body">
                <div class="app-logo">
                    <AppLogo/>
                </div>

                <div class="version-info">
                    <p class="version">Version {dataStore.appVersion}</p>
                    {#if dataStore.checkingUpdates}
                        <p class="update-status checking">Checking for updates...</p>
                    {:else if dataStore.updateCheckError}
                        <div class="error-container">
                            <p class="update-status error">Update check failed</p>
                            <button class="retry-button" onclick={() => dataStore.checkUpdates()}>
                                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                     stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M21 2v6h-6M3 12a9 9 0 0 1 15-6.7L21 8M3 22v-6h6M21 12a9 9 0 0 1-15 6.7L3 16"/>
                                </svg>
                                Retry
                            </button>
                        </div>
                    {:else if dataStore.updateCount > 0}
                        <p class="update-status warning">
                            {dataStore.updateCount} {dataStore.updateCount === 1 ? 'update' : 'updates'} behind latest
                            release ({dataStore.latestVersion})
                        </p>
                    {:else if dataStore.latestVersion}
                        <p class="update-status success">
                            You are using the latest version
                        </p>
                    {/if}
                </div>

                <div class="info-section">
                    <p class="description">A user-friendly Parquet file viewer.</p>
                    <button
                            class="link-text"
                            onclick={() => openLink('https://pinkparquet.com')}
                    >https://pinkparquet.com
                    </button>
                </div>

                <div class="links">
                    <button
                            class="link-button"
                            onclick={() => openLink('https://github.com/marepilc/pink-parquet')}
                    >
                        <svg
                                width="20"
                                height="20"
                                viewBox="0 0 24 24"
                                fill="currentColor"
                                xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                    d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.17 6.839 9.49.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.463-1.11-1.463-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.578 9.578 0 0112 6.836c.85.004 1.705.114 2.504.336 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.138 20.167 22 16.418 22 12c0-5.523-4.477-10-10-10z"
                            />
                        </svg>
                        <span>View on GitHub</span>
                    </button>

                    <button
                            class="link-button sponsor"
                            onclick={() => openLink('https://github.com/sponsors/marepilc')}
                    >
                        <svg
                                width="20"
                                height="20"
                                viewBox="0 0 24 24"
                                fill="currentColor"
                                xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                    d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"
                            />
                        </svg>
                        <span>Become a Sponsor</span>
                    </button>
                </div>

                <div class="footer">
                    <p>© {copyrightYear} Marek Pilczuk. Licensed under MIT.</p>
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    #about-title {
        color: var(--accent);
        font-family: var(--font-display), serif;
    }

    .modal-backdrop {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.5);
        backdrop-filter: blur(4px);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        animation: fadeIn 0.2s ease-out;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    .modal-content {
        background: var(--surface-2);
        border: 1px solid var(--surface-5);
        border-radius: 0.5rem;
        width: 90%;
        max-width: 500px;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
        animation: slideIn 0.2s ease-out;
    }

    @keyframes slideIn {
        from {
            transform: translateY(-20px);
            opacity: 0;
        }
        to {
            transform: translateY(0);
            opacity: 1;
        }
    }

    .modal-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 1.5rem;
        border-bottom: 1px solid var(--surface-4);
    }

    .modal-header h2 {
        font-size: 1.25rem;
        font-weight: 600;
        color: var(--ink-5);
        margin: 0;
    }

    .close-button {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 2rem;
        height: 2rem;
        border-radius: 0.25rem;
        background: transparent;
        border: none;
        color: var(--ink-3);
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
            background: var(--surface-3);
            color: var(--accent);
        }
    }

    .modal-body {
        padding: 2rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }

    .app-logo {
        width: 20rem;
    }

    .version {
        font-size: 0.875rem;
        color: var(--ink-3);
        margin: 0;
    }

    .version-info {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.25rem;
    }

    .update-status {
        font-size: 0.75rem;
        font-weight: 500;
        margin: 0;
    }

    .update-status.warning {
        color: var(--color-warning);
    }

    .update-status.success {
        color: var(--surface-9);
    }

    .update-status.checking {
        color: var(--ink-2);
        font-style: italic;
    }

    .update-status.error {
        color: var(--color-error);
    }

    .error-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5rem;
    }

    .retry-button {
        display: flex;
        align-items: center;
        gap: 0.25rem;
        background: var(--surface-3);
        border: 1px solid var(--surface-5);
        border-radius: 0.25rem;
        color: var(--ink-3);
        padding: 0.25rem 0.5rem;
        font-size: 0.7rem;
        cursor: pointer;
        transition: all 0.2s;

        &:hover {
            background: var(--surface-4);
            color: var(--accent);
        }
    }

    .info-section {
        text-align: center;
        margin: 1rem 0;
    }

    .description {
        font-size: 0.875rem;
        color: var(--ink-4);
        line-height: 1.5;
        margin: 0;
    }

    .links {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        width: 100%;
        margin-top: 1rem;
    }

    .link-button {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        padding: 0.75rem 1.5rem;
        border-radius: 0.375rem;
        border: 1px solid var(--surface-5);
        background: var(--surface-3);
        color: var(--ink-5);
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .link-button:hover {
        background: var(--surface-4);
        border-color: var(--surface-6);
        transform: translateY(-1px);
    }

    .link-button.sponsor {
        background: var(--accent);
        border-color: var(--accent);
        border-color: oklch(from var(--accent) calc(l * 0.8) c h);
        color: white;
    }

    .link-button.sponsor:hover {
        filter: brightness(1.1);
        background: oklch(from var(--accent) calc(l * 0.9) c h);
    }

    .footer {
        margin-top: 1rem;
        padding-top: 1rem;
        border-top: 1px solid var(--surface-4);
        width: 100%;
        text-align: center;
    }

    .footer p {
        font-size: 0.85rem;
        color: var(--ink-2);
        margin: 0;
    }

    .description {
        text-align: center;
        font-size: 1rem;
    }

    .link-text {
        margin-top: 0.5rem;
        color: var(--accent);
        font-family: var(--font-display), serif;
        cursor: pointer;
        border: none;
        background: transparent;

        &:hover {
            text-decoration: underline;
        }
    }
</style>
