<script lang="ts">
  import type { Snippet } from 'svelte'
  import { Spring } from 'svelte/motion'

  interface Props {
    ariaLabel: string
    hoverRotate?: number
    onClick?: () => void
    children: Snippet
  }

  let { ariaLabel, hoverRotate = 90, onClick, children }: Props = $props()

  const rotation = new Spring(0, { stiffness: 0.1, damping: 0.3 })
  const scale = new Spring(1, { stiffness: 0.1, damping: 0.4 })
  const opacity = new Spring(1, { stiffness: 0.1, damping: 0.4 })

  function handleMouseEnter() {
    rotation.target = hoverRotate
    // Pulse effect using spring oscillation
    scale.target = 1.05
    opacity.target = 0.7
    setTimeout(() => {
      scale.target = 1
      opacity.target = 1
    }, 300)
  }

  function handleMouseLeave() {
    rotation.target = 0
    scale.target = 1
    opacity.target = 1
  }
</script>

<button
  class="app-round-btn"
  onclick={onClick}
  onmouseenter={handleMouseEnter}
  onmouseleave={handleMouseLeave}
  aria-label={ariaLabel}
  tabindex="-1"
>
  <span
    class="pulse-circle"
    style="transform: scale({scale.current}); opacity: {opacity.current};"
  ></span>
  <span class="icon" style="transform: rotate({rotation.current}deg);">
    {@render children()}
  </span>
</button>

<style>
  .app-round-btn {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    margin-top: auto;
    margin-bottom: auto;
    border: var(--surface-5) solid 1px;
    border-radius: calc(infinity * 1px);
    cursor: pointer;
    color: var(--ink-5);
    background: linear-gradient(145deg, var(--surface-6), var(--surface-2));
    box-shadow:
      inset 0.1rem 0.1rem 0.2rem var(--surface-9),
      inset -0.1rem -0.1rem 0.2rem var(--surface-1);
    &:hover {
      color: var(--accent);
      transition: all 0.5s ease;
    }
    &::before {
      position: absolute;
      content: '';
      inset: 0;
      width: inherit;
      height: inherit;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      border-radius: inherit;
      background: linear-gradient(145deg, var(--surface-6), var(--surface-2));
      box-shadow:
        0.15rem 0.15rem 0.5rem var(--surface-8),
        -0.15rem -0.15rem 0.5rem var(--surface-2);
      z-index: -1;
    }
  }

  .pulse-circle {
    position: absolute;
    inset: 0;
    background: radial-gradient(circle, var(--surface-1), var(--surface-5));
    border-radius: calc(infinity * 1px);
    pointer-events: none;
  }

  .icon {
    position: relative;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
