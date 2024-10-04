export function formatToLocalDateTime(isoString: string): string {
    const date = new Date(isoString)
    if (isNaN(date.getTime())) {
        throw new Error('Invalid date string')
    }

    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0') // Months are 0-based
    const day = String(date.getDate()).padStart(2, '0')
    const hours = String(date.getHours()).padStart(2, '0')
    const minutes = String(date.getMinutes()).padStart(2, '0')

    return `${year}-${month}-${day} ${hours}:${minutes}`
}

export function bytesForHuman(bytes: number): string {
    if (bytes < 0) {
        throw new Error('Bytes value cannot be negative')
    }
    const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB']
    let index = 0
    let value = bytes

    while (value >= 1024 && index < units.length - 1) {
        value /= 1024
        index++
    }

    return `${value.toFixed(2)} ${units[index]}`
}
