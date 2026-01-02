import { settingsStore, type FontFamily } from './settingsStore.svelte'

// Font store for managing monospace font selection
const FONTS: FontFamily[] = ['Iosevka', 'Argon', 'Krypton', 'Neon', 'Radon', 'Xenon']

export const fontStore = {
  get currentFont(): FontFamily {
    return settingsStore.settings.fontFamily
  },

  get currentFontSize(): number {
    return settingsStore.settings.fontSize
  },

  async cycleFont() {
    const currentIndex = FONTS.indexOf(settingsStore.settings.fontFamily)
    const nextIndex = (currentIndex + 1) % FONTS.length
    await settingsStore.setFontFamily(FONTS[nextIndex])
  },

  async decreaseFontSize() {
    const currentSize = settingsStore.settings.fontSize
    await settingsStore.setFontSize(currentSize - 1)
  },

  async increaseFontSize() {
    const currentSize = settingsStore.settings.fontSize
    await settingsStore.setFontSize(currentSize + 1)
  }
}
