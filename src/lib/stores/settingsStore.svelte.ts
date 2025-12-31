import { invoke } from '@tauri-apps/api/core'

export type FontFamily = 'Iosevka' | 'Argon' | 'Krypton' | 'Neon' | 'Radon' | 'Xenon'

export interface Settings {
  theme: 'light' | 'dark'
  fontFamily: FontFamily
  fontSize: number
}

const DEFAULT_SETTINGS: Settings = {
  theme: 'dark',
  fontFamily: 'Iosevka',
  fontSize: 14,
}

// Module-level state
let settings = $state<Settings>(DEFAULT_SETTINGS)
let loaded = $state(false)

export const settingsStore = {
  get settings() {
    return settings
  },

  get loaded() {
    return loaded
  },

  async load() {
    try {
      const loadedSettings = await invoke<Settings>('load_settings')
      settings = { ...DEFAULT_SETTINGS, ...loadedSettings }
      loaded = true
    } catch (error) {
      console.error('Failed to load settings:', error)
      settings = DEFAULT_SETTINGS
      loaded = true
    }
  },

  async setTheme(theme: 'light' | 'dark') {
    settings.theme = theme
    await this.save()
    this.applyTheme()
  },

  async setFontFamily(fontFamily: FontFamily) {
    settings.fontFamily = fontFamily
    await this.save()
    this.applyFont()
  },

  async setFontSize(fontSize: number) {
    // Clamp font size between 12 and 18
    settings.fontSize = Math.max(12, Math.min(18, fontSize))
    await this.save()
    this.applyFontSize()
  },

  async save() {
    try {
      await invoke('save_settings', { settings })
    } catch (error) {
      console.error('Failed to save settings:', error)
    }
  },

  applyTheme() {
    if (settings.theme === 'dark') {
      document.documentElement.classList.add('dark')
      document.documentElement.classList.remove('light')
      document.documentElement.style.colorScheme = 'dark'
    } else {
      document.documentElement.classList.add('light')
      document.documentElement.classList.remove('dark')
      document.documentElement.style.colorScheme = 'light'
    }
  },

  applyFont() {
    const fontFamily = settings.fontFamily === 'Iosevka' ? 'IosevkaMP' : `Monaspace ${settings.fontFamily}`
    document.documentElement.style.setProperty('--font-mono', `'${fontFamily}', monospace`)
  },

  applyFontSize() {
    document.documentElement.style.setProperty('--font-size-mono', `${settings.fontSize}px`)
  },

  // Initialize on first load
  async init() {
    if (!loaded) {
      await this.load()
      this.applyTheme()
      this.applyFont()
      this.applyFontSize()
    }
  },
}
