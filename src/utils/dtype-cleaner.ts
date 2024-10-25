export function dtypeCleaner(dtype: string): string {
    if (dtype.startsWith('Categorical')) {
        return 'Categorical'
    } else if (dtype.startsWith('Datetime')) {
        return 'Datetime'
    } else if (dtype.startsWith('Duration')) {
        return 'Duration'
    } else if (dtype.startsWith('List')) {
        return 'List'
    } else if (dtype.startsWith('Struct')) {
        return 'Struct'
    } else if (dtype.startsWith('Enum')) {
        return 'Enum'
    } else {
        return dtype
    }
}
