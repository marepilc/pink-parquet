import {
    readTextFile,
    writeTextFile,
    mkdir,
    BaseDirectory,
    exists,
} from '@tauri-apps/plugin-fs'

export const useConfigStore = defineStore({
    id: 'configStore',
    state: () => ({
        theme: 'light',
        uiFont: 'System UI',
        tableFont: 'Ubuntu Mono',
    }),
    actions: {
        async loadConfig() {
            try {
                // check if the config file exists
                const configExists = await exists('config.json', {
                    baseDir: BaseDirectory.AppConfig,
                })
                if (configExists) {
                    // read the config file
                    const config = await readTextFile('config.json', {
                        baseDir: BaseDirectory.AppConfig,
                    })
                    this.theme = JSON.parse(config).theme
                    this.uiFont = JSON.parse(config).uiFont
                    this.tableFont = JSON.parse(config).tableFont
                } else {
                    // create the config file
                    await mkdir('', {
                        baseDir: BaseDirectory.AppConfig,
                        recursive: true,
                    })
                    await writeTextFile(
                        'config.json',
                        JSON.stringify(this.$state),
                        {
                            baseDir: BaseDirectory.AppConfig,
                        }
                    )
                }
            } catch (e) {
                console.error(e)
            }
        },
        async saveConfig() {
            try {
                await writeTextFile(
                    'config.json',
                    JSON.stringify(this.$state),
                    {
                        baseDir: BaseDirectory.AppConfig,
                    }
                )
            } catch (e) {
                console.error(e)
            }
        },
        toggleTheme() {
            this.theme = this.theme === 'light' ? 'dark' : 'light'
            this.saveConfig()
        },
        setUIFont(font: string) {
            this.uiFont = font
            this.saveConfig()
        },
        setTableFont(font: string) {
            this.tableFont = font
            this.saveConfig()
        },
    },
})
