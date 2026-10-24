<script module lang="ts">
    let draggedSessionId: string | null = null
    let lastTargetId: string | null = null
    let lastPosition: 'before' | 'after' | null = null
    let indicatorElement: HTMLElement | null = null

    function clearDropIndicator() {
        if (indicatorElement) {
            indicatorElement.classList.remove('drop-before', 'drop-after')
            indicatorElement = null
        }
    }
</script>

<script lang="ts">
    import CloseIcon from '$lib/components/icons/CloseIcon.svelte'
    import SqlIcon from '$lib/components/icons/SqlIcon.svelte'
    import {dataStore} from '$lib/stores/dataStore.svelte'

    // import ParquetIcon from '$lib/components/icons/ParquetIcon.svelte'

    interface Props {
        session: { id: string; path: string | null; name: string; isQueryResult?: boolean; isQueryEditor?: boolean }
        isActive: boolean
        onClick: () => void
        onClose: () => void
    }

    let {session, isActive, onClick, onClose}: Props = $props()
    let isEditing = $state(false)
    let editedName = $state('')
    let suppressClick = $state(false)
    let pointerDragging = $state(false)
    let startX = 0
    let startY = 0

    function cleanupPointerDrag(target: HTMLElement) {
        target.style.opacity = '1'
        draggedSessionId = null
        pointerDragging = false
    }

    function handlePointerDown(e: PointerEvent) {
        if (isEditing || e.button !== 0) return

        const target = e.target as HTMLElement
        if (target.closest('.tab-close') || target.closest('.rename-input')) return

        const currentTarget = e.currentTarget as HTMLElement
        draggedSessionId = session.id
        startX = e.clientX
        startY = e.clientY
        suppressClick = false

        const onPointerMove = (event: PointerEvent) => {
            if (!draggedSessionId) return

            const dx = Math.abs(event.clientX - startX)
            const dy = Math.abs(event.clientY - startY)
            if (!pointerDragging && dx + dy < 4) return

            pointerDragging = true
            suppressClick = true
            currentTarget.style.opacity = '0.5'
            event.preventDefault()

            const hovered = document
                .elementFromPoint(event.clientX, event.clientY)
                ?.closest('.tab[data-session-id]') as HTMLElement | null

            const targetId = hovered?.dataset.sessionId
            if (!hovered || !targetId || targetId === draggedSessionId) {
                clearDropIndicator()
                lastTargetId = null
                lastPosition = null
                return
            }

            const rect = hovered.getBoundingClientRect()
            const position: 'before' | 'after' =
                event.clientX < rect.left + rect.width / 2 ? 'before' : 'after'

            if (indicatorElement !== hovered) {
                clearDropIndicator()
                indicatorElement = hovered
            }

            indicatorElement.classList.toggle('drop-before', position === 'before')
            indicatorElement.classList.toggle('drop-after', position === 'after')

            if (targetId !== lastTargetId || position !== lastPosition) {
                dataStore.reorderSession(draggedSessionId, targetId, position)
                lastTargetId = targetId
                lastPosition = position
            }
        }

        const onPointerUp = () => {
            document.removeEventListener('pointermove', onPointerMove)
            document.removeEventListener('pointerup', onPointerUp)
            document.removeEventListener('pointercancel', onPointerUp)
            clearDropIndicator()
            lastTargetId = null
            lastPosition = null
            cleanupPointerDrag(currentTarget)
        }

        document.addEventListener('pointermove', onPointerMove)
        document.addEventListener('pointerup', onPointerUp)
        document.addEventListener('pointercancel', onPointerUp)
    }

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

    function handleTabClick(e: MouseEvent) {
        if (suppressClick) {
            e.preventDefault()
            e.stopPropagation()
            suppressClick = false
            return
        }
        onClick()
    }
</script>

<div
        role="button"
        tabindex="-1"
        class="tab"
        class:tab-active={isActive}
        onclick={handleTabClick}
        onkeydown={handleKeydown}
        data-session-id={session.id}
        draggable="false"
        onpointerdown={handlePointerDown}
        style="-webkit-app-region: no-drag; app-region: no-drag; pointer-events: auto; user-select: none;"
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
        <div class="display-mode-container">
            {#if session.isQueryResult || session.isQueryEditor}
                <SqlIcon size={12} className="query-icon"/>
            {/if}
            <span class="name-text">{session.name}</span>
        </div>
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
        <CloseIcon size={12}/>
    </button>
</div>

<style>
    .tab {
        position: relative;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        min-width: 7.5rem;
        max-width: 14rem;
        padding: 0 0.75rem;
        border-bottom: 2px solid transparent;
        cursor: grab;
        color: var(--text-muted);
        transition: color 150ms ease, background-color 150ms ease;
        user-select: none;

        &:hover {
            color: var(--text);

            .tab-title {
                transform: translateX(-0.75rem);
            }

            .tab-close {
                opacity: 1;
                transition: opacity 300ms ease 200ms;
            }
        }
    }

    .tab:active {
        cursor: grabbing;
    }

    :global(.drop-before) {
        box-shadow: inset 3px 0 0 var(--accent);
    }

    :global(.drop-after) {
        box-shadow: inset -3px 0 0 var(--accent);
    }

    .tab-active {
        background: var(--bg);
        color: var(--text);
        border-bottom-color: var(--accent);
    }

    .tab-title {
        position: relative;
        left: 0.75rem;
        flex: 1;
        overflow: hidden;
        transition: all 200ms ease;
    }

    .display-mode-container {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.375rem;
        width: 100%;
    }

    :global(.query-icon) {
        flex-shrink: 0;
        color: var(--accent-text);
        opacity: 0.8;
    }

    .name-text {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        text-align: center;
        font-size: 0.75rem;
        font-weight: 500;
    }

    .rename-input {
        width: 100%;
        background: var(--panel-2);
        border: 1px solid var(--accent);
        color: var(--text);
        font-size: 0.75rem;
        font-family: inherit;
        font-weight: 500;
        padding: 2px 4px;
        border-radius: 4px;
        text-align: center;
        outline: none;
    }

    .tab-close {
        color: var(--text-muted);
        opacity: 0;
        transition: opacity 200ms ease;
        background-color: var(--chip);
        border: none;
        padding: 0.2rem;
        border-radius: 50%;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .tab-close:hover {
        color: var(--text);
    }
</style>
