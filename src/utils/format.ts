// Helper function to pad a number with leading zeros
function padNumber(num: number, length: number): string {
    return String(num).padStart(length, '0')
}

// Centralized date formatting function
function formatDateToParts(date: Date): {
    year: string
    month: string
    day: string
    hours: string
    minutes: string
    seconds: string
    milliseconds: string
} {
    const year = String(date.getFullYear())
    const month = padNumber(date.getMonth() + 1, 2) // Months are 0-based
    const day = padNumber(date.getDate(), 2)
    const hours = padNumber(date.getHours(), 2)
    const minutes = padNumber(date.getMinutes(), 2)
    const seconds = padNumber(date.getSeconds(), 2)
    const milliseconds = padNumber(date.getMilliseconds(), 3)

    return { year, month, day, hours, minutes, seconds, milliseconds }
}

// Formats an ISO string to 'YYYY-MM-DD HH:mm'
export function formatToLocalDateTime(isoString: string): string {
    const date = new Date(isoString)
    if (isNaN(date.getTime())) {
        throw new Error('Invalid date string')
    }

    const { year, month, day, hours, minutes } = formatDateToParts(date)

    return `${year}-${month}-${day} ${hours}:${minutes}`
}

// Formats a Date object to 'YYYY-MM-DDTHH:mm:ss.SSS'
export function formatDate(date: Date): string {
    const { year, month, day, hours, minutes, seconds, milliseconds } =
        formatDateToParts(date)

    return `${year}-${month}-${day}T${hours}:${minutes}:${seconds}.${milliseconds}`
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

export function numberThousandsSeparator(num: number | string): string {
    // Convert the number to a string and use a regular expression to add commas
    return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',')
}
