<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { getCurrentWindow } from '@tauri-apps/api/window'
  import AppLogo from '$lib/components/AppLogo.svelte'
  import RoundBtn from '$lib/components/RoundBtn.svelte'
  import CloseIcon from '$lib/components/icons/CloseIcon.svelte'
  import MaximizeIcon from '$lib/components/icons/MaximizeIcon.svelte'
  import MinimizeIcon from '$lib/components/icons/MinimizeIcon.svelte'

  let isMaximized = $state(false)
  const appWindow = getCurrentWindow()

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
>
  <div class="title-bar-content">
    <div class="logo-container">
      <div id="app-logo">
        <AppLogo />
      </div>
    </div>
  </div>

  <div id="app-buttons-container" class="no-drag">
    <RoundBtn ariaLabel="Minimize" onClick={minimize} hoverRotate={180}>
      <MinimizeIcon size={20} />
    </RoundBtn>

    <RoundBtn
      ariaLabel={isMaximized ? 'Restore' : 'Maximize'}
      onClick={toggleMaximize}
      hoverRotate={180}
    >
      <MaximizeIcon size={20} />
    </RoundBtn>

    <RoundBtn ariaLabel="Close" onClick={close} hoverRotate={180}>
      <CloseIcon size={20} />
    </RoundBtn>
  </div>
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

  .logo-container {
    display: flex;
    align-items: center;
  }

  #app-logo {
    display: flex;
    align-items: center;
    width: 10rem;
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
  }

  .no-drag {
    -webkit-app-region: no-drag;
  }
</style>
