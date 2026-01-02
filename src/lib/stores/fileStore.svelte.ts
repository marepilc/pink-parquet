interface FileState {
  isOpen: boolean
  filePath: string
}

class FileStore {
  #state = $state<FileState>({
    isOpen: false,
    filePath: '',
  })

  get isOpen() {
    return this.#state.isOpen
  }

  get filePath() {
    return this.#state.filePath
  }

  openFile(path: string) {
    this.#state.isOpen = true
    this.#state.filePath = path
  }

  closeFile() {
    this.#state.isOpen = false
    this.#state.filePath = ''
  }

  reset() {
    this.#state.isOpen = false
    this.#state.filePath = ''
  }
}

export const fileStore = new FileStore()
