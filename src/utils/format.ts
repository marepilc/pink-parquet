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

export function removeExtension(fileName: string): string {
    const lastDotIndex = fileName.lastIndexOf('.')
    if (lastDotIndex === -1) {
        return fileName
    }
    return fileName.slice(0, lastDotIndex)
}

export function convertToDate(days) {
    const milliseconds = days * 24 * 60 * 60 * 1000 // Convert days to milliseconds
    return new Date(milliseconds).toISOString().split('T')[0] // Outputs just the date portion
}

export function convertToDatetime(microseconds) {
    const milliseconds = Math.floor(microseconds / 1000) // Convert microseconds to milliseconds
    const date = new Date(milliseconds)

    // Extract date components
    const year = date.getUTCFullYear()
    const month = (date.getUTCMonth() + 1).toString().padStart(2, '0') // Months are 0-indexed
    const day = date.getUTCDate().toString().padStart(2, '0')
    const hours = date.getUTCHours().toString().padStart(2, '0')
    const minutes = date.getUTCMinutes().toString().padStart(2, '0')
    const seconds = date.getUTCSeconds().toString().padStart(2, '0')

    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`
}

export function convertToTime(nanoseconds) {
    const milliseconds = Math.floor(nanoseconds / 1e6) // Convert nanoseconds to milliseconds
    const date = new Date(milliseconds)

    const hours = date.getUTCHours()
    const minutes = date.getUTCMinutes()
    const seconds = date.getUTCSeconds()
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
}

export function convertToDuration(nanoseconds) {
    const milliseconds = nanoseconds / 1e6 // Convert nanoseconds to milliseconds
    const seconds = Math.floor(milliseconds / 1000)
    const remainingMilliseconds = milliseconds % 1000

    return `${seconds}s ${remainingMilliseconds}ms`
}
