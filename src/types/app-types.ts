export interface Column {
    name: string
    dtype: string
}

export type Shape = [number, number]

export interface Sorting {
    column: string
    ascending: boolean
}

export interface TableData {
    shape: [number, number]
    columns: Array<{ name: string; dtype: string }>
    rows: string[][]
}

export interface FileMetadata {
    fileName: string
    createdAt: string
    modifiedAt: string
    size: number
}
