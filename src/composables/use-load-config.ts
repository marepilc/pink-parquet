import {
    readTextFile,
    writeTextFile,
    mkdir,
    BaseDirectory,
    exists,
} from '@tauri-apps/plugin-fs'

export default async function () {
    const defaultConfig = {
        theme: 'light',
    }

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
            return JSON.parse(config)
        } else {
            // create the config file
            await mkdir('', {
                baseDir: BaseDirectory.AppConfig,
                recursive: true,
            })
            await writeTextFile('config.json', JSON.stringify(defaultConfig), {
                baseDir: BaseDirectory.AppConfig,
            })
            return defaultConfig
        }
    } catch (e) {
        console.error(e)
        return defaultConfig
    }
}
