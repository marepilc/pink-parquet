<script lang="ts">
    import ToolbarBtn from '$lib/components/ToolbarBtn.svelte'
    import ToggleSwitch from '$lib/components/ToggleSwitch.svelte'
    import {dataStore} from '$lib/stores/dataStore.svelte'
    import {settingsStore} from '$lib/stores/settingsStore.svelte'
    import {fontStore} from '$lib/stores/fontStore.svelte'
    import SqlIcon from '$lib/components/icons/SqlIcon.svelte'
    import FileOpenIcon from '$lib/components/icons/FileOpenIcon.svelte'
    import SaveIcon from '$lib/components/icons/SaveIcon.svelte'
    import InfoIcon from '$lib/components/icons/InfoIcon.svelte'
    import IosevkaIcon from '$lib/components/icons/IosevkaIcon.svelte'
    import ArgonIcon from '$lib/components/icons/ArgonIcon.svelte'
    import KryptonIcon from '$lib/components/icons/KryptonIcon.svelte'
    import NeonIcon from '$lib/components/icons/NeonIcon.svelte'
    import RadonIcon from '$lib/components/icons/RadonIcon.svelte'
    import XenonIcon from '$lib/components/icons/XenonIcon.svelte'
    import MinusIcon from '$lib/components/icons/MinusIcon.svelte'
    import PlusIcon from '$lib/components/icons/PlusIcon.svelte'
    import AboutModal from '$lib/components/AboutModal.svelte'
    import {open, save} from '@tauri-apps/plugin-dialog'
    import {invoke} from '@tauri-apps/api/core'
    import {goto} from '$app/navigation'

    let showAboutModal = $state(false)

    const fontIcons = {
        Iosevka: IosevkaIcon,
        Argon: ArgonIcon,
        Krypton: KryptonIcon,
        Neon: NeonIcon,
        Radon: RadonIcon,
        Xenon: XenonIcon,
    }

    let CurrentFontIcon = $derived(fontIcons[fontStore.currentFont])
    let isMinFontSize = $derived(fontStore.currentFontSize <= 12)
    let isMaxFontSize = $derived(fontStore.currentFontSize >= 18)

    function toggleSql() {
        dataStore.isSqlTabActive = !dataStore.isSqlTabActive
    }

    function openAbout() {
        showAboutModal = true
    }

    async function handleThemeChange(isDark: boolean) {
        const newTheme = isDark ? 'dark' : 'light'
        await settingsStore.setTheme(newTheme)
    }

    async function loadParquetFile(filePath: string, forceReload: boolean = false) {
        await dataStore.loadParquetFile(filePath, forceReload, goto)
    }

    async function handleOpenFile() {
        try {
            const files = await open({
                multiple: true,
                filters: [
                    {
                        name: 'Parquet',
                        extensions: ['parquet'],
                    },
                ],
            })

            if (files && Array.isArray(files)) {
                for (const file of files) {
                    await loadParquetFile(file)
                }
            } else if (files) {
                await loadParquetFile(files as string)
            }
        } catch (error) {
            console.error('Error opening file:', error)
        }
    }

    async function handleSaveAs() {
        try {
            // Determine default path based on current view
            let defaultPath: string | undefined

            if (dataStore.isSqlTabActive) {
                // SQL mode - use "untitled.parquet"
                defaultPath = 'untitled.parquet'
            } else {
                // File mode - use active session name (handles renamed files)
                const activeSession = dataStore.sessions.find(
                    (s) => s.id === dataStore.activeSessionId
                )
                if (activeSession) {
                    // Extract directory from original path and combine with current name
                    const originalPath = activeSession.path
                    const pathSeparator = originalPath.includes('/') ? '/' : '\\'
                    const lastSeparatorIndex = originalPath.lastIndexOf(pathSeparator)

                    if (lastSeparatorIndex >= 0) {
                        const directory = originalPath.substring(0, lastSeparatorIndex + 1)
                        // Ensure the name has .parquet extension
                        const name = activeSession.name.endsWith('.parquet')
                            ? activeSession.name
                            : `${activeSession.name}.parquet`
                        defaultPath = directory + name
                    } else {
                        // Fallback if no separator found
                        defaultPath = activeSession.name.endsWith('.parquet')
                            ? activeSession.name
                            : `${activeSession.name}.parquet`
                    }
                }
            }

            const filePath = await save({
                defaultPath,
                filters: [
                    {
                        name: 'Parquet',
                        extensions: ['parquet'],
                    },
                ],
            })

            if (filePath) {
                await invoke('save_parquet', {filePath})
                console.log('File saved successfully to:', filePath)

                // If the saved file is already open in any session, reload that session
                const existingSession = dataStore.sessions.find((s) => s.path === filePath)
                if (existingSession) {
                    await loadParquetFile(filePath, true)
                }
            }
        } catch (error) {
            console.error('Error saving file:', error)
        }
    }
</script>

<div class="toolbar-container">
    <ToolbarBtn ariaLabel="Open File" onClick={handleOpenFile}>
        <FileOpenIcon size={32}/>
    </ToolbarBtn>

    <ToolbarBtn
            ariaLabel="Save As"
            onClick={handleSaveAs}
            disabled={!dataStore.data}
    >
        <SaveIcon size={32}/>
    </ToolbarBtn>

    <ToolbarBtn
            ariaLabel="SQL Editor"
            onClick={toggleSql}
            active={dataStore.isSqlTabActive}
            disabled={!dataStore.hasData}
    >
        <SqlIcon size={32}/>
    </ToolbarBtn>

    <ToolbarBtn ariaLabel="About" onClick={openAbout}>
        <InfoIcon size={32}/>
    </ToolbarBtn>

    <div class="font-controls">
        <ToolbarBtn ariaLabel="Change Font" onClick={() => fontStore.cycleFont()}>
            <CurrentFontIcon size={32}/>
        </ToolbarBtn>

        <div class="font-size-controls">
            <button
                    class="font-size-btn"
                    aria-label="Decrease Font Size"
                    onclick={() => fontStore.decreaseFontSize()}
                    disabled={isMinFontSize}
                    tabindex="-1"
            >
                <MinusIcon size={12}/>
            </button>
            <button
                    class="font-size-btn"
                    aria-label="Increase Font Size"
                    onclick={() => fontStore.increaseFontSize()}
                    disabled={isMaxFontSize}
                    tabindex="-1"
            >
                <PlusIcon size={12}/>
            </button>
        </div>
    </div>

    <ToggleSwitch
            checked={settingsStore.settings.theme === 'dark'}
            onChange={handleThemeChange}
            ariaLabel="Toggle dark mode"
    />
</div>

<AboutModal bind:isOpen={showAboutModal}/>

<style>
    .toolbar-container {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5rem;
        background-image: linear-gradient(
                to bottom,
                var(--surface-4),
                var(--surface-5)
        );
        padding: 0.5rem 0;
        border-radius: 0.25rem;
        height: 100%;
        border: 1px groove var(--surface-9);
        border: 1px groove oklch(from var(--surface-9) l c h / 0.5);
        /*box-shadow:*/
        /*  0.05rem 0.05rem 0.5rem rgb(0 0 0 / 0.25),*/
        /*  -0.05rem -0.05rem 0.3rem rgb(255 255 255 / 0.5);*/

        &::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            border: 2px solid transparent;
            background: conic-gradient(
                    from 170deg at 50% 50%,
                    var(--surface-1),
                    var(--surface-5) 10%,
                    var(--surface-5) 40%,
                    var(--surface-1) 100%
            ) border-box;
            background: conic-gradient(
                    from 170deg at 50% 50%,
                    oklch(from var(--surface-1) l c h / 0.5),
                    var(--surface-5) 10%,
                    var(--surface-5) 40%,
                    oklch(from var(--surface-1) l c h / 0.5) 100%
            ) border-box;
            mask: linear-gradient(black) border-box,
            linear-gradient(white) padding-box;
            mask-composite: subtract;
            border-radius: inherit;
            filter: blur(1rem);
        }
    }

    .font-controls {
        position: relative;
        z-index: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.25rem;
    }

    .font-size-controls {
        display: flex;
        flex-direction: row;
        gap: 0.25rem;
    }

    .font-size-btn {
        position: relative;
        z-index: 10;
        display: flex;
        justify-content: center;
        align-items: center;
        width: 1.25rem;
        height: 1.25rem;
        padding: 0.125rem;
        border: 1px solid var(--surface-2);
        border-radius: 0.25rem;
        background-image: radial-gradient(var(--surface-2), var(--surface-4));
        background-size: 400% 400%;
        background-position: 0 0;
        transition: background-position 0.3s ease-in-out;
        color: var(--ink-5);
        cursor: pointer;

        &:hover:not(:disabled) {
            background-position: 50% 50%;
        }

        &:disabled {
            opacity: 0.3;
        }
    }
</style>
