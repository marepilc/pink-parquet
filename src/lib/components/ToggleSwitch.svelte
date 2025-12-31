<script lang="ts">
  import { onMount } from 'svelte'

  interface Props {
    checked: boolean
    onChange: (checked: boolean) => void
    ariaLabel: string
  }

  let { checked = false, onChange, ariaLabel }: Props = $props()
  let mounted = $state(false)

  onMount(() => {
    // Enable transitions after mount and initial state settling
    // Delay ensures theme settings have loaded before transitions are enabled
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        mounted = true
      })
    })
  })

  function handleToggle() {
    onChange(!checked)
  }
</script>

<button
  class="toggle-switch"
  class:checked
  class:mounted
  onclick={handleToggle}
  role="switch"
  aria-checked={checked}
  aria-label={ariaLabel}
  tabindex="-1"
>
  <span class="toggle-track">
    <span class="toggle-thumb"></span>
  </span>
</button>

<style>
  .toggle-switch {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.25rem;
    background: transparent;
    border: none;
    cursor: pointer;
  }

  .toggle-track {
    position: relative;
    display: flex;
    align-items: center;
    width: 3rem;
    height: 1.5rem;
    background-image: url('../../assets/img/day.svg');
    border: 1px solid var(--surface-8);
    border-radius: calc(infinity * 1px);
  }

  .toggle-switch.mounted .toggle-track {
    transition: background-color 0.3s ease;
  }

  .toggle-thumb {
    position: absolute;
    left: 0.15rem;
    width: 1.1rem;
    height: 1.1rem;
    background: var(--surface-3);
    border: 1px solid var(--surface-6);
    border-radius: calc(infinity * 1px);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .toggle-switch.mounted .toggle-thumb {
    transition: transform 0.3s ease;
  }

  .toggle-switch.checked .toggle-track {
    background-image: url('../../assets/img/night.svg');
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
  }

  .toggle-switch.checked .toggle-thumb {
    transform: translateX(1.55rem);
  }

  .toggle-switch:hover .toggle-thumb {
    background: var(--surface-2);
  }
</style>
