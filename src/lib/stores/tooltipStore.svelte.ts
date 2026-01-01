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

  show(targetElement: HTMLElement, tooltipText: string) {
    if (hideTimeout) {
      clearTimeout(hideTimeout)
      hideTimeout = null
    }

    showTimeout = setTimeout(() => {
      const rect = targetElement.getBoundingClientRect()
      text = tooltipText
      x = rect.left + rect.width / 2
      y = rect.bottom + 8
      visible = true
    }, 500) // Show after 500ms hover
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

    visible = false
  },

  hideImmediate() {
    if (showTimeout) clearTimeout(showTimeout)
    if (hideTimeout) clearTimeout(hideTimeout)
    visible = false
  },
}
