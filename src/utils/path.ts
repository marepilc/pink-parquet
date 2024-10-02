export function extractPath(dragEventPaths: string[]): string | null {
    const parquetPaths = dragEventPaths.filter((path) =>
        path.endsWith('parquet')
    )
    if (parquetPaths.length === 0) {
        return null
    } else {
        return parquetPaths[0]
    }
}
