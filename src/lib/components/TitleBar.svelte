<script lang="ts">
    import {invoke} from '@tauri-apps/api/core'
    import {type} from '@tauri-apps/plugin-os'
    import {onMount} from 'svelte'
    import {getCurrentWindow} from '@tauri-apps/api/window'
    import AppLogo from '$lib/components/AppLogo.svelte'
    import RoundBtn from '$lib/components/RoundBtn.svelte'
    import CloseIcon from '$lib/components/icons/CloseIcon.svelte'
    import MaximizeIcon from '$lib/components/icons/MaximizeIcon.svelte'
    import MinimizeIcon from '$lib/components/icons/MinimizeIcon.svelte'

    let isMaximized = $state(false)
    let isMacOS = $state(false)
    let isLinux = $state(false)
    const appWindow = getCurrentWindow()

    onMount(async () => {
        const osType = await type()
        isMacOS = osType === 'macos'
        isLinux = osType === 'linux'
    })

    // Check initially maximized state
    async function checkMaximized() {
        isMaximized = await invoke('is_maximized')
    }

    // Listen for window resize events to update the maximize button
    appWindow.listen('tauri://resize', async () => {
        isMaximized = await invoke('is_maximized')
    })

    // Initialize
    checkMaximized()

    async function minimize() {
        await invoke('minimize_window')
    }

    async function toggleMaximize() {
        if (isMaximized) {
            await invoke('unmaximize_window')
        } else {
            await invoke('maximize_window')
        }
    }

    async function close() {
        await invoke('close_window')
    }

    // Handle double-click to maximize/restore
    function handleDoubleClick() {
        toggleMaximize()
    }

    async function handleMouseDown(event: MouseEvent) {
        // Only use JavaScript dragging on Linux (Windows/Mac use CSS data-tauri-drag-region)
        if (!isLinux) return

        if (event.button === 0) { // Left click
            // Prevent dragging if clicking on buttons
            const target = event.target as HTMLElement
            if (target.closest('#app-buttons-container')) {
                return
            }
            try {
                await appWindow.startDragging()
            } catch (error) {
                console.error('Failed to start dragging:', error)
            }
        }
    }
</script>

<div
        id="title-bar-container"
        role="banner"
        ondblclick={handleDoubleClick}
        onmousedown={handleMouseDown}
        data-tauri-drag-region
>
    <div class="title-bar-content" class:macos={isMacOS} data-tauri-drag-region>
        <div class="logo-container" data-tauri-drag-region>
            <div id="app-logo" data-tauri-drag-region>
                <AppLogo/>
            </div>
        </div>
    </div>

    {#if !isMacOS}
        <div id="app-buttons-container" class="no-drag">
            <RoundBtn ariaLabel="Minimize" onClick={minimize} hoverRotate={180}>
                <MinimizeIcon size={20}/>
            </RoundBtn>

            <RoundBtn
                    ariaLabel={isMaximized ? 'Restore' : 'Maximize'}
                    onClick={toggleMaximize}
                    hoverRotate={180}
            >
                <MaximizeIcon size={20}/>
            </RoundBtn>

            <RoundBtn ariaLabel="Close" onClick={close} hoverRotate={180}>
                <CloseIcon size={20}/>
            </RoundBtn>
        </div>
    {/if}
</div>

<!--suppress CssUnknownProperty -->
<style>
    #title-bar-container {
        z-index: 100;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
    }

    .title-bar-content {
        display: flex;
        height: 100%;
        flex: 1;
        align-items: center;
        gap: 0.75rem;
        padding-left: 1rem;
        padding-right: 1rem;
    }

    .title-bar-content.macos {
        padding-left: 80px;
    }

    .logo-container {
        display: flex;
        align-items: center;
        height: 100%;
    }

    #app-logo {
        display: flex;
        align-items: center;
        width: 10rem;
    }

    #app-logo :global(svg) {
        width: 100%;
        height: auto;
        display: block;
    }

    #app-buttons-container {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        height: 100%;
        gap: 0.5rem;
        padding-right: 0.5rem;
        pointer-events: auto;
    }

    .no-drag {
        -webkit-app-region: no-drag;
        app-region: no-drag;
    }
</style>
