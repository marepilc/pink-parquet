# Pink Parquet - Svelte Edition

This is an improved version of the [pink-parquet](https://github.com/marepilc/pink-parquet) application, rebuilt with modern technologies.

## Tech Stack

- **Frontend**: SvelteKit 2.49.2 with Svelte 5.0 (using runes)
- **Backend**: Tauri 2 with Rust
- **Build Tool**: Vite 7.2.7
- **Styling**: Tailwind CSS 4.1.18
- **Data Processing**: Polars 0.44 (Rust DataFrame library)
- **TypeScript**: ~5.6.2

## Reference Application

Original application: [pink-parquet](https://github.com/marepilc/pink-parquet) (Vue + Tauri)

## Current Features

- ✅ Custom title bar with window controls on Windows (minimize, maximize, close)
- ✅ Native system title bar on macOS and Linux for better OS integration
- ✅ File upload via file dialog or drag-and-drop
- ✅ Parquet file reading using Polars
- ✅ Basic data display (file info, columns, data preview)
- ✅ Footer with file path and close button
- ✅ Reactive state management using Svelte 5 runes

## Planned Features

### Data Table Component

- Replace current data preview with a full-featured table component
- Table should be similar to the reference repository implementation
- Initial load: 100 rows
- Infinite scroll: load additional batches as user scrolls
- Efficient rendering for large datasets

### SQL Query Support

- Add SQL query functionality for Parquet files
- Use Polars SQL capabilities on the Rust backend
- Query editor component in the UI
- Display query results in the same table component

## Architecture Notes

### Stores Pattern (Svelte 5 Runes)

Use **module-level `$state` variables** for reactive stores in `.svelte.ts` files:

```typescript
// ✅ Correct pattern
let data = $state<DataType | null>(null)

export const store = {
  get data() {
    return data
  },
  setData(newData: DataType) {
    data = newData
  },
}
```

**Do NOT use** class-based `$state` with getters - this breaks reactivity:

```typescript
// ❌ Incorrect - reactivity doesn't work
class Store {
  #data = $state<DataType | null>(null)
  get data() {
    return this.#data
  }
}
```

### File Structure

```
src/
├── lib/
│   ├── components/
│   │   ├── TitleBar.svelte          - Custom window controls
│   │   ├── FileUpload.svelte        - File selection/drag-drop
│   │   ├── DataView.svelte          - Data display (to be replaced with table)
│   │   └── Footer.svelte            - File path display
│   └── stores/
│       ├── fileStore.svelte.ts      - File state management
│       └── dataStore.svelte.ts      - Parquet data state
└── routes/
    └── +page.svelte                 - Main page with conditional rendering

src-tauri/
└── src/
    └── lib.rs                       - Rust backend with read_parquet command
```

## Development Status

### Completed

1. ✅ Custom title bar with styled window controls (circular buttons with pulse effects)
2. ✅ File upload component with drag-and-drop support
3. ✅ Parquet file reading on Rust backend using Polars
4. ✅ Basic data display with file info, columns, and preview
5. ✅ Reactive state management using Svelte 5 runes

### Next Steps

1. Implement table component with infinite scroll (100 rows initial, batch loading)
2. Add SQL query functionality
3. Improve table performance for large datasets
4. Add column sorting, filtering
5. Add data type indicators and column statistics
