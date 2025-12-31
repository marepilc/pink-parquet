<script lang="ts">
  import type { Snippet } from 'svelte'

  interface Props {
    ariaLabel: string
    onClick?: () => void
    active?: boolean
    disabled?: boolean
    children: Snippet
  }

  let {
    ariaLabel,
    onClick,
    active = false,
    disabled = false,
    children,
  }: Props = $props()
</script>

<div class="btn-wrap">
  <button
    class="toolbar-btn"
    class:active
    aria-label={ariaLabel}
    onclick={onClick}
    tabindex="-1"
    {disabled}
  >
    <span>{@render children?.()}</span>
  </button>
</div>

<style>
  .btn-wrap {
    z-index: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 0.125rem 0.25rem;
    width: fit-content;
  }

  .toolbar-btn {
    display: flex;
    justify-content: center;
    align-items: center;
    aspect-ratio: 1;
    width: 3rem;
    padding: 0.25rem;
    border: 1px solid var(--surface-2);
    border-radius: 0.25rem;
    background-image: radial-gradient(var(--surface-2), var(--surface-4));
    background-size: 400% 400%;
    background-position: 0 0;
    transition: background-position 0.3s ease-in-out;
    color: var(--ink-5);
    cursor: pointer;
    font-size: var(--text-sm);

    & > span {
      display: flex;
      justify-content: center;
      align-items: center;
    }

    &:hover:not(:disabled) {
      background-position: 50% 50%;
    }

    &.active {
      border-color: var(--accent);
    }

    &:disabled {
      opacity: 0.5;
      cursor: default;
    }
  }
</style>
