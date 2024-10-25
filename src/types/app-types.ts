export interface Column {
    name: string
    dtype: string
    compression?: string
}

export type Shape = [number, number]

export interface Sorting {
    column: string
    ascending: boolean
}

export interface Filtering {
    column: string
    condition: Condition
    value: string | number | [string, string]
}

export interface TableData {
    shape: [number, number]
    columns: Array<{ name: string; dtype: string }>
    rows: string[][]
    height: number
}

export interface ColumnMetadata {
    name: string
    compression: string
}

export interface FileMetadata {
    columns: ColumnMetadata[]
    createdAt: string
    fileName: string
    fileSize: number
    modifiedAt: string
    numRows: number
    rowGroups?: number
}

export enum Condition {
    eq = '==',
    neq = '!=',
    gt = '>',
    ge = '>=',
    lt = '<',
    le = '<=',
    between = 'between',
    equals = 'equals',
    contains = 'contains',
    containsCaseInsensitive = 'contains_case_insensitive',
    different = 'different',
    isNull = 'is_null',
    isNotNull = 'is_not_null',
}

export interface ParquetData {
    shape: Shape
    columns: Column[]
    rows: string[][]
    metadata: FileMetadata
}
