<script lang="ts">
    import {open} from '@tauri-apps/plugin-dialog'
    import {dataStore} from '$lib/stores/dataStore.svelte'
    import {goto} from '$app/navigation'
    import FileOpenIcon from '$lib/components/icons/FileOpenIcon.svelte'

    async function loadParquetFile(filePath: string) {
        await dataStore.loadParquetFile(filePath, false, goto)
    }

    async function handleFileSelect() {
        try {
            const files = await open({
                multiple: true,
                filters: [
                    {
                        name: 'Data Files',
                        extensions: ['parquet', 'csv'],
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
        To open a Parquet or CSV file, click the <span class="no-wrap"
    >button <span class="icon"><FileOpenIcon size={32}/></span></span
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
        user-select: none;

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
