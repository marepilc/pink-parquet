<script lang="ts">
    import IconParquetFile from '$lib/components/icons/IconParquetFile.svelte'

    interface Props {
        isVisible: boolean
        hasData: boolean
    }

    let {isVisible, hasData}: Props = $props()
</script>

{#if isVisible}
    <div class="overlay" class:transparent={hasData} class:opaque={!hasData}>
        <div class="content">
      <span class="icon">
        <IconParquetFile/>
      </span>
            <span class="text">Drop file to open</span>
        </div>
    </div>
{/if}

<style>
    .overlay {
        position: fixed;
        inset: 0;
        z-index: 50;
        display: flex;
        align-items: center;
        justify-content: center;
        border: 4px dashed var(--color-primary-500);
        pointer-events: none;
    }

    .overlay.transparent {
        background-color: var(--color-primary-500);
        opacity: 0.1;
        backdrop-filter: blur(4px);
    }

    .overlay.opaque {
        background-color: var(--color-surface-700);
        opacity: 0.95;
        backdrop-filter: blur(12px);
    }

    .content {
        display: flex;
        flex-direction: column;
        align-items: center;
        color: var(--color-primary-500);
    }

    .icon {
        width: 8rem;
        height: 8rem;
        transform: scale(1.1);
        transition: transform 300ms;
    }

    .text {
        margin-top: 1.5rem;
        font-size: 1.875rem;
        font-weight: 600;
    }
</style>
