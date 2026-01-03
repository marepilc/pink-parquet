// Global tooltip state
let text = $state<string>('')
let x = $state(0)
let y = $state(0)
let visible = $state(false)
let showTimeout: ReturnType<typeof setTimeout> | null = null
let hideTimeout: ReturnType<typeof setTimeout> | null = null

export const tooltipStore = {
    get text() {
        return text
    },
    get x() {
        return x
    },
    get y() {
        return y
    },
    get visible() {
        return visible
    },

    show(targetElement: HTMLElement, tooltipText: string, mouseX?: number, mouseY?: number) {
        if (hideTimeout) {
            clearTimeout(hideTimeout)
            hideTimeout = null
        }

        if (visible) {
            return
        }

        if (showTimeout) {
            clearTimeout(showTimeout)
        }

        showTimeout = setTimeout(() => {
            const rect = targetElement.getBoundingClientRect()
            text = tooltipText
            if (mouseX !== undefined && mouseY !== undefined) {
                x = mouseX
                y = mouseY + 15 // Slightly less offset
            } else {
                x = rect.left + rect.width / 2
                y = rect.bottom + 8
            }
            visible = true
        }, 1000) // Show after 1000ms hover
    },

    hide() {
        if (showTimeout) {
            clearTimeout(showTimeout)
            showTimeout = null
        }

        if (hideTimeout) {
            clearTimeout(hideTimeout)
            hideTimeout = null
        }

        if (visible) {
            hideTimeout = setTimeout(() => {
                visible = false
            }, 100)
        }
    },

    hideImmediate() {
        if (showTimeout) clearTimeout(showTimeout)
        if (hideTimeout) clearTimeout(hideTimeout)
        visible = false
    },
}
