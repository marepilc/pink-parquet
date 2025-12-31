<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog'
  import { dataStore } from '$lib/stores/dataStore.svelte'
  import { invoke } from '@tauri-apps/api/core'
  import { goto } from '$app/navigation'
  import FileOpenIcon from '$lib/components/icons/FileOpenIcon.svelte'

  async function loadParquetFile(filePath: string) {
    // Check if file is already open
    const existingSession = dataStore.sessions.find((s) => s.path === filePath)
    if (existingSession) {
      // File already open, just switch to it
      dataStore.activeSessionId = existingSession.id
      // Navigate to /app if not already there
      await goto('/app')
      return
    }

    const sessionId = dataStore.addSession(filePath)
    dataStore.setLoading(true, sessionId, false)

    try {
      const data = await invoke('get_data', {
        filePath,
        sorting: null,
      })
      dataStore.setData(data as any, sessionId, false)

      // Navigate to /app after successful load
      await goto('/app')
    } catch (error) {
      console.error('Error loading Parquet file:', error)
      dataStore.setError(String(error), sessionId)
    }
  }

  async function handleFileSelect() {
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
</script>

<div class="file-upload-container">
  <p>
    To open a Parquet file, click the <span class="no-wrap"
      >button <span class="icon"><FileOpenIcon size={32} /></span></span
    > in the toolbar or drag and drop a file into the application window.
  </p>
</div>

<style>
  .file-upload-container {
    display: flex;
    height: 100%;
    width: 50%;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    margin: auto;

    p {
      margin: 0;
      line-height: 1.5;
      color: var(--ink-5);
      font-size: 1.5rem;
    }

    .no-wrap {
      white-space: nowrap;
    }

    .icon {
      display: inline-block;
      position: relative;
      top: 0.25rem;
    }
  }
</style>
