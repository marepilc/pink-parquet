<script lang="ts">
  import CloseIcon from '$lib/components/icons/CloseIcon.svelte'
  import { dataStore } from '$lib/stores/dataStore.svelte'

  // import ParquetIcon from '$lib/components/icons/ParquetIcon.svelte'

  interface Props {
    session: { id: string; path: string; name: string }
    isActive: boolean
    onClick: () => void
    onClose: () => void
  }

  let { session, isActive, onClick, onClose }: Props = $props()
  let isEditing = $state(false)
  let editedName = $state('')

  function handleDblClick() {
    editedName = session.name
    isEditing = true
  }

  function handleBlur() {
    saveRename()
  }

  function handleNameKeydown(e: KeyboardEvent) {
    // Stop event from bubbling to parent tab
    e.stopPropagation()

    if (e.key === 'Enter') {
      saveRename()
    } else if (e.key === 'Escape') {
      isEditing = false
    }
  }

  function saveRename() {
    if (isEditing) {
      const trimmed = editedName.trim()
      if (trimmed && trimmed !== session.name) {
        dataStore.renameSession(session.id, trimmed)
      }
      isEditing = false
    }
  }

  function selectAll(node: HTMLInputElement) {
    node.focus()
    node.select()
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault()
      onClick()
    }
  }

  function handleCloseClick(e: MouseEvent) {
    e.stopPropagation()
    onClose()
  }
</script>

<div
  role="button"
  tabindex="-1"
  class="tab"
  class:tab-active={isActive}
  onclick={onClick}
  onkeydown={handleKeydown}
>
  <!--  <ParquetIcon size={24} className="text-ink-50" />-->
  {#snippet editMode()}
    <input
      type="text"
      class="rename-input"
      bind:value={editedName}
      onblur={handleBlur}
      onkeydown={handleNameKeydown}
      use:selectAll
    />
  {/snippet}

  {#snippet displayMode()}
    {session.name}
  {/snippet}

  <span
    class="tab-title"
    role="textbox"
    tabindex="0"
    aria-label="Rename session"
    ondblclick={handleDblClick}
    onkeydown={(e) => {
      if (e.key === 'Enter') {
        handleDblClick()
      }
    }}
  >
    {#if isEditing}
      {@render editMode()}
    {:else}
      {@render displayMode()}
    {/if}
  </span>
  <button class="tab-close" onclick={handleCloseClick} aria-label="Close tab">
    <CloseIcon size={12} />
  </button>

  {#if isActive}
    <div class="tab-indicator"></div>
  {/if}
</div>

<style>
  .tab {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    height: 2rem;
    min-width: 7.5rem;
    max-width: 12.5rem;
    padding: 0 0.75rem;
    border-radius: 0.5rem;
    border: 1px solid var(--surface-6);
    cursor: pointer;
    color: var(--ink-5);
    transition: all 200ms ease;

    &:hover {
      border-color: var(--surface-7);

      .tab-title {
        transform: translateX(-0.75rem);
      }

      .tab-close {
        opacity: 1;
        transition: opacity 300ms ease 200ms;
      }
    }
  }

  .tab-active {
    background-image: linear-gradient(
      145deg,
      var(--surface-3),
      var(--surface-2)
    );
    color: var(--accent);
    border-color: var(--surface-9);
  }

  .tab-title {
    position: relative;
    left: 0.75rem;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    text-align: center;
    font-size: 0.75rem;
    font-weight: 500;
    transition: all 200ms ease;
  }

  .rename-input {
    width: 100%;
    background: var(--surface-1);
    border: 1px solid var(--accent);
    color: var(--ink-5);
    font-size: 0.75rem;
    font-family: inherit;
    font-weight: 500;
    padding: 2px 4px;
    border-radius: 4px;
    text-align: center;
    outline: none;
  }

  .tab-close {
    color: var(--ink-1);
    opacity: 0;
    transition: opacity 200ms ease;
    background-color: var(--surface-3);
    border: none;
    padding: 0.2rem;
    border-radius: 50%;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .tab-close:hover {
    color: var(--ink-7);
  }

  .tab-indicator {
    position: absolute;
    top: -0.25rem;
    left: 0;
    width: 100%;
    height: 0.25rem;
    box-shadow: 0 -2px 4px rgba(0, 0, 0, 0.1);
  }
</style>
