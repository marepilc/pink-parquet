<script lang="ts">
  export let size: number = 12
  export let className: string = ''
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  class={className}
  aria-hidden="true"
  focusable="false"
>
  <circle cx="12" cy="12" r="8" />
  <path d="M8 8l8 8" />
</svg>
