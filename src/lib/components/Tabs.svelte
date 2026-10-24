<script lang="ts">
    import {dataStore} from '$lib/stores/dataStore.svelte'
    import Tab from '$lib/components/Tab.svelte'
    import {onMount} from 'svelte'

    let containerElement: HTMLDivElement
    let showScrollButtons = $state(false)
    let canScrollLeft = $state(false)
    let canScrollRight = $state(false)

    function checkScroll() {
        if (containerElement) {
            const {scrollLeft, scrollWidth, clientWidth} = containerElement
            // Always show scroll buttons when there's overflow
            showScrollButtons = scrollWidth > clientWidth

            // Track if scrolling is possible in each direction (for disabled state)
            canScrollLeft = scrollLeft > 0
            canScrollRight = scrollLeft < scrollWidth - clientWidth - 1 // -1 for sub-pixel rounding
        }
    }

    function scroll(direction: 'left' | 'right') {
        if (containerElement) {
            const scrollAmount = 200
            containerElement.scrollBy({
                left: direction === 'left' ? -scrollAmount : scrollAmount,
                behavior: 'smooth',
            })
        }
    }

    onMount(() => {
        checkScroll()
        const observer = new ResizeObserver(checkScroll)
        observer.observe(containerElement)
        return () => observer.disconnect()
    })

    // Re-check scroll state when sessions change (tabs added/removed)
    $effect(() => {
        // Track sessions length to trigger effect
        dataStore.sessions.length
        // Use requestAnimationFrame to ensure DOM has updated
        requestAnimationFrame(() => {
            checkScroll()
        })
    })
</script>

<div class="tabs-outer-container">
    {#if showScrollButtons}
        <button
                class="scroll-button left"
                class:disabled={!canScrollLeft}
                onclick={() => scroll('left')}
                disabled={!canScrollLeft}
                title="Scroll left"
                tabindex="-1"
        >
            <svg
                    width="16"
                    height="16"
                    viewBox="0 0 16 16"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
            >
                <path
                        d="M10 4L6 8L10 12"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                />
            </svg>
        </button>
    {/if}

    <div
            class="tabs-container"
            bind:this={containerElement}
            onscroll={checkScroll}
            style="-webkit-app-region: no-drag; app-region: no-drag; pointer-events: auto;"
    >
        <div class="tabs-list">
            {#each dataStore.sessions as session (session.id)}
                <Tab
                        {session}
                        isActive={dataStore.activeSessionId === session.id}
                        onClick={() => {
                            dataStore.activeSessionId = session.id
                            // Ensure data view is shown for non-editor sessions,
                            // and SQL view is shown for editor sessions
                            dataStore.isSqlTabActive = !!session.isQueryEditor
                        }}
                        onClose={() => {
            const wasLastSession = dataStore.sessions.length === 1
            dataStore.removeSession(session.id)

            // Navigate to landing if no sessions left
            if (wasLastSession) {
              // Reset all state
              dataStore.isSqlTabActive = false
              dataStore.clearData()

              // Force full page reload to landing
              window.location.href = '/'
            }
          }}
                />
            {/each}
        </div>
    </div>

    {#if showScrollButtons}
        <button
                class="scroll-button right"
                class:disabled={!canScrollRight}
                onclick={() => scroll('right')}
                disabled={!canScrollRight}
                title="Scroll right"
                tabindex="-1"
        >
            <svg
                    width="16"
                    height="16"
                    viewBox="0 0 16 16"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
            >
                <path
                        d="M6 4L10 8L6 12"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                />
            </svg>
        </button>
    {/if}
</div>

<style>
    .tabs-outer-container {
        position: relative;
        display: flex;
        align-items: stretch;
        width: 100%;
        overflow: hidden;
        background: var(--panel);
        border-bottom: 1px solid var(--border);
    }

    .tabs-container {
        position: relative;
        display: flex;
        align-items: stretch;
        height: 2.375rem;
        flex-grow: 1;
        flex-shrink: 1;
        min-width: 0;
        justify-content: flex-start;
        overflow-x: auto;
        padding: 0 0.5rem;
    }

    /* Hide scrollbar */
    .tabs-container::-webkit-scrollbar {
        display: none;
    }

    .tabs-container {
        -ms-overflow-style: none;
        scrollbar-width: none;
    }

    .tabs-list {
        display: flex;
        align-items: stretch;
        gap: 0.125rem;
    }

    .scroll-button {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 2rem;
        flex-shrink: 0;
        color: var(--text-muted);
        background: none;
        border: 0;
        cursor: pointer;
    }

    .scroll-button:hover:not(:disabled) {
        background-color: var(--hover);
        color: var(--text);
    }

    .scroll-button:disabled {
        opacity: 0.35;
        cursor: default;
    }
</style>
