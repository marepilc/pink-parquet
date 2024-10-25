<script setup lang="ts">
import {
    ref,
    computed,
    watch,
    onMounted,
    onBeforeUnmount,
    nextTick,
    defineExpose,
} from 'vue'

const props = defineProps({
    mode: {
        type: String as PropType<'click' | 'hover'>,
        default: 'click',
    },
    disabled: {
        type: Boolean,
        default: false,
    },
    openDelay: {
        type: Number,
        default: 0,
    },
    closeDelay: {
        type: Number,
        default: 0,
    },
})

const emits = defineEmits(['update:open'])

const popoverOpen = ref(false)
const popoverRef = ref<HTMLElement | null>(null)
const activatorRef = ref<HTMLElement | null>(null)

let openTimeout: ReturnType<typeof setTimeout> | null = null
let closeTimeout: ReturnType<typeof setTimeout> | null = null

function openPopover() {
    if (openTimeout) {
        clearTimeout(openTimeout)
        openTimeout = null
    }
    if (closeTimeout) {
        clearTimeout(closeTimeout)
        closeTimeout = null
    }
    openTimeout = setTimeout(() => {
        popoverOpen.value = true
        emits('update:open', true)
        nextTick(() => {
            positionPopover()
        })
    }, props.openDelay)
}

function closePopover() {
    if (openTimeout) {
        clearTimeout(openTimeout)
        openTimeout = null
    }
    if (closeTimeout) {
        clearTimeout(closeTimeout)
        closeTimeout = null
    }
    closeTimeout = setTimeout(() => {
        popoverOpen.value = false
        emits('update:open', false)
    }, props.closeDelay)
}

function togglePopover() {
    if (popoverOpen.value) {
        closePopover()
    } else {
        openPopover()
    }
}

defineExpose({
    openPopover,
    closePopover,
    togglePopover,
})

function positionPopover() {
    if (popoverRef.value && activatorRef.value) {
        const activatorRect = activatorRef.value.getBoundingClientRect()
        const popoverEl = popoverRef.value

        const popoverHeight = popoverEl.offsetHeight
        const popoverWidth = popoverEl.offsetWidth

        const viewportHeight = window.innerHeight
        const viewportWidth = window.innerWidth

        // Calculate positions
        let top = activatorRect.bottom + window.scrollY + 4
        let left =
            activatorRect.left +
            window.scrollX +
            activatorRect.width / 2 -
            popoverWidth / 2

        // Adjust positioning if not enough space
        if (left < 4) {
            left = 4 // Add some padding from the left edge
        } else if (left + popoverWidth > viewportWidth) {
            left = viewportWidth - popoverWidth - 4 // Add some padding from the right edge
        }

        if (top + popoverHeight > viewportHeight + window.scrollY) {
            // Not enough space below, position above the activator
            top = activatorRect.top + window.scrollY - popoverHeight - 4
        }

        // Apply positioning styles
        popoverEl.style.position = 'absolute'
        popoverEl.style.top = `${top}px`
        popoverEl.style.left = `${left}px`
        popoverEl.style.zIndex = '10'
    }
}

function handleClickOutside(event: MouseEvent) {
    if (
        popoverRef.value &&
        activatorRef.value &&
        !popoverRef.value.contains(event.target as Node) &&
        !activatorRef.value.contains(event.target as Node)
    ) {
        closePopover()
    }
}

function onMouseEnter() {
    if (props.mode === 'hover') {
        openPopover()
    }
}

function onMouseLeave() {
    if (props.mode === 'hover') {
        closePopover()
    }
}

watch(popoverOpen, (isOpen) => {
    if (isOpen) {
        document.addEventListener('click', handleClickOutside)
    } else {
        document.removeEventListener('click', handleClickOutside)
    }
})

onBeforeUnmount(() => {
    document.removeEventListener('click', handleClickOutside)
})
</script>

<template>
    <div class="inline-flex flex-col items-center">
        <!-- Activator Slot -->
        <div
            ref="activatorRef"
            :class="{ 'cursor-not-allowed': disabled }"
            @click="
                !disabled && props.mode === 'click' ? togglePopover() : null
            "
            @mouseenter="onMouseEnter"
            @mouseleave="onMouseLeave"
        >
            <slot name="activator" :open="openPopover" :close="closePopover">
                <!-- Default activator content -->
                <button :disabled="disabled" class="your-button-classes">
                    Open
                </button>
            </slot>
        </div>

        <!-- Popover Content -->
        <div
            v-if="popoverOpen"
            ref="popoverRef"
            class="scale-95 transform opacity-0 transition-all duration-300 ease-out"
            :class="popoverOpen ? 'scale-100 opacity-100' : ''"
            style="position: absolute; z-index: 10"
            @mouseenter="onMouseEnter"
            @mouseleave="onMouseLeave"
        >
            <div
                class="bg-form-light dark:bg-form-dark min-w-60 rounded-md border border-surface-300 shadow-md dark:border-surface-800"
            >
                <slot :open="openPopover" :close="closePopover"></slot>
            </div>
        </div>
    </div>
</template>
