<script lang="ts">
  import { dataStore } from '$lib/stores/dataStore.svelte'
  import Tab from '$lib/components/Tab.svelte'
  import { onMount } from 'svelte'

  let containerElement: HTMLDivElement
  let showLeftScroll = $state(false)
  let showRightScroll = $state(false)

  function checkScroll() {
    if (containerElement) {
      const { scrollLeft, scrollWidth, clientWidth } = containerElement
      showLeftScroll = scrollLeft > 0
      showRightScroll = scrollLeft < scrollWidth - clientWidth - 1 // -1 for sub-pixel rounding
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
</script>

<div class="tabs-outer-container">
  {#if showLeftScroll}
    <button
      class="scroll-button left"
      onclick={() => scroll('left')}
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
  >
    <div class="tabs-list">
      {#each dataStore.sessions as session (session.id)}
        <Tab
          {session}
          isActive={!dataStore.isSqlTabActive &&
            dataStore.activeSessionId === session.id}
          onClick={() => {
            dataStore.activeSessionId = session.id
            dataStore.isSqlTabActive = false
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

  {#if showRightScroll}
    <button
      class="scroll-button right"
      onclick={() => scroll('right')}
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
    align-items: center;
    width: 100%;
    overflow: hidden;
  }

  .tabs-container {
    position: relative;
    display: flex;
    align-items: center;
    height: 2.5rem;
    flex-grow: 1;
    flex-shrink: 1;
    min-width: 0;
    justify-content: flex-start;
    gap: 0.25rem;
    overflow-x: auto;
    padding: 0.25rem 0.5rem;
    background-image: linear-gradient(
      to right,
      var(--surface-4),
      var(--surface-3)
    );
    border-radius: 0.25rem;
    border: 1px groove oklch(from var(--surface-9) l c h / 0.5);
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
    gap: 0.25rem;
  }

  .scroll-button {
    position: absolute;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
    height: 2rem;
    width: 1.5rem;
    border-radius: 0.25rem;
    color: oklch(from var(--ink-1) l c h / 0.7);
    background: var(--surface-3);
    border: 1px solid var(--surface-6);
    cursor: pointer;
    transition: all 200ms ease;
    z-index: 10;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
  }

  .scroll-button:hover {
    background-color: var(--surface-2);
    color: var(--accent);
  }

  .scroll-button.left {
    left: 0.5rem;
  }

  .scroll-button.right {
    right: 0.5rem;
  }
</style>
