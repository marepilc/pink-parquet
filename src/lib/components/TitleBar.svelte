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
    const appWindow = getCurrentWindow()

    onMount(async () => {
        isMacOS = (await type()) === 'macos'
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
</script>

<div
        id="title-bar-container"
        class="drag-region"
        role="banner"
        ondblclick={handleDoubleClick}
        data-tauri-drag-region
>
    <div class="title-bar-content" class:macos={isMacOS}>
        <div class="logo-container">
            <div id="app-logo">
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
        z-index: 50;
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
    }

    #app-logo {
        display: flex;
        align-items: center;
        width: 10rem;
        height: 100%;
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
    }

    .drag-region {
        -webkit-app-region: drag;
        app-region: drag;
    }

    .no-drag {
        -webkit-app-region: no-drag;
        app-region: no-drag;
    }
</style>
