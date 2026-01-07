<script lang="ts">
    import {dataStore} from '$lib/stores/dataStore.svelte'
    import {tooltipStore} from '$lib/stores/tooltipStore.svelte'
    import {untrack} from 'svelte'

    let containerElement = $state<HTMLElement | null>(null)
    let textElement = $state<HTMLElement | null>(null)
    let isOverflowing = $state(false)

    $effect(() => {
        if (containerElement && textElement) {
            const checkOverflow = () => {
                // To check if it WOULD overflow, we temporarily remove constraints
                // or check scrollWidth if it's currently constrained.
                // If .path-text has max-width: 100%, scrollWidth will be its natural width.
                const textWidth = textElement!.scrollWidth
                const containerWidth = containerElement!.clientWidth
                isOverflowing = textWidth > containerWidth
            }

            checkOverflow()

            const resizeObserver = new ResizeObserver(checkOverflow)
            resizeObserver.observe(containerElement)
            // No need to observe textElement because it's a child and we care about container size
            // but path change is important.

            return () => resizeObserver.disconnect()
        }
    })

    // Re-check overflow when the path changes
    $effect(() => {
        if (dataStore.activeSession?.path) {
            // Wait for next tick so DOM is updated with new path
            untrack(() => {
                setTimeout(() => {
                    if (containerElement && textElement) {
                        const textWidth = textElement.scrollWidth
                        const containerWidth = containerElement.clientWidth
                        isOverflowing = textWidth > containerWidth
                    }
                }, 0)
            })
        }
    })
</script>

<footer id="app-footer">
    <div
            bind:this={containerElement}
            class="path-container"
            class:can-scroll={isOverflowing}
            role="status"
            onmouseenter={(e) => {
      if (dataStore.hasData && dataStore.activeSession) {
        tooltipStore.show(e.currentTarget, dataStore.activeSession.path, e.clientX, e.clientY)
      }
    }}
            onmousemove={(e) => {
      if (dataStore.hasData && dataStore.activeSession) {
        if (tooltipStore.visible) {
          tooltipStore.hide()
        } else {
          tooltipStore.show(e.currentTarget, dataStore.activeSession.path, e.clientX, e.clientY)
        }
      }
    }}
            onmouseleave={() => tooltipStore.hide()}
    >
        {#if dataStore.hasData && dataStore.activeSession}
            <span bind:this={textElement} class="path-text">{dataStore.activeSession.path}</span>
        {:else}
            No parquet file loaded.
        {/if}
    </div>
</footer>

<style>
    #app-footer {
        padding: 0.25rem 0.5rem;
        border-top: 1px var(--surface-3) solid;
        display: flex;
        align-items: center;
        color: var(--ink-3);
        font-size: 0.75rem;
        font-style: italic;
        user-select: none;
        min-width: 0;
    }

    .path-container {
        overflow: hidden;
        white-space: nowrap;
        flex: 1;
        min-width: 0;
        position: relative;
        display: flex;
        align-items: center;
    }

    .path-container.can-scroll {
        cursor: help;
    }

    .path-text {
        display: inline-block;
        white-space: nowrap;
        position: relative;
        left: 0;
        transform: translateX(0);
        transition: transform 0.6s ease-in-out, left 0.6s ease-in-out, max-width 0s 0.6s, overflow 0s 0.6s, text-overflow 0s 0.6s;
        max-width: 100%;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .path-container.can-scroll:hover .path-text {
        max-width: none;
        overflow: visible;
        text-overflow: clip;
        left: 100%;
        transform: translateX(-100%);
        transition: transform 0.6s ease-in-out, left 0.6s ease-in-out, max-width 0s, overflow 0s, text-overflow 0s;
    }
</style>
