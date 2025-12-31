<script lang="ts">
  export let size: number = 12
  export let className: string = ''
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="currentColor"
  class={className}
  aria-hidden="true"
  focusable="false"
>
  <path d="M5 5v14h2v2H3V3h4v2H5Zm12 2v2H7V7h10Zm0 4v2H7v-2h10Zm0 4v2H7v-2h10Zm2 4V5h-2V3h4v18h-4v-2h2Z" />
</svg>
