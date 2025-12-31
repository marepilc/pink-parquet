<script lang="ts">
  let { size = 24, className = '' } = $props<{
    size?: number
    className?: string
  }>()
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="currentColor"
  stroke="none"
  class={className}
>
  <path
    d="M4.697,13.102l0,5.439l-2.697,0l0,-13.082l2.697,0l0,5.457l1.493,0l1.903,-2.814l0,-2.643l2.688,0l0,3.091l-2.412,3.27l2.412,3.629l0,3.091l-2.912,0l0,-2.348l-1.867,-3.091l-1.306,0Zm10.986,-1.48l0,6.919l-2.616,0l0,-9.57l1.962,0l0.56,1.243l3.929,-1.458l2.482,2.276l0,1.873l-2.608,0.367l0,-1.425l-0.78,-0.744l-2.93,0.518Z"
  />
</svg>
