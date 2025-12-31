# PinkParquet Development Plan

## Svelte-based Parquet Viewer Desktop Application

**Last Updated**: 2025-12-13

---

## Current State

- ✅ **Tech Stack Already in Place**: Tauri v2 + SvelteKit + Svelte 5.0 + TypeScript
- ✅ **Build System**: Vite 6.0 with HMR
- ✅ **Desktop Framework**: Tauri v2 configured and working
- ⚠️ **Parquet Functionality**: Not yet implemented (template/skeleton only)

---

## Project Vision

Build a fast, user-friendly desktop application for viewing and analyzing Parquet files with a modern Svelte-based UI.

---

## Development Phases

### **Phase 1: Core Parquet Functionality** 🎯 _Priority: HIGH_

#### 1.1 Backend - Rust Parquet Integration

- [ ] Add Rust parquet dependencies to `src-tauri/Cargo.toml`
  - `arrow` and `parquet` crates from Apache Arrow
  - Consider `polars` as an alternative (faster, more batteries-included)
- [ ] Implement Tauri commands for file operations:
  - `open_parquet_file()` - Open file dialog and read parquet file
  - `get_parquet_metadata()` - Extract schema, row count, column info
  - `get_parquet_data()` - Read data with pagination support
  - `search_parquet()` - Search/filter capabilities
- [ ] Add file dialog plugin: `tauri-plugin-dialog`
- [ ] Implement error handling for corrupted/invalid files
- [ ] Add support for reading compressed parquet files (Snappy, GZIP, LZ4, etc.)

**Files to modify:**

- [src-tauri/Cargo.toml](src-tauri/Cargo.toml) - Add dependencies
- [src-tauri/src/lib.rs](src-tauri/src/lib.rs) - Implement commands
- [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json) - Add dialog plugin

#### 1.2 Frontend - Basic UI Components

- [ ] Create file picker UI component
  - Drag-and-drop zone for parquet files
  - "Open File" button using Tauri dialog
- [ ] Design and implement data table component
  - Virtual scrolling for large datasets (use `svelte-virtual` or similar)
  - Column headers with metadata
  - Sortable columns
  - Resizable columns
- [ ] Create metadata viewer panel
  - Schema information
  - Row/column counts
  - File size and compression info
  - Column data types
- [ ] Implement loading states and progress indicators
- [ ] Add error handling UI (error messages, invalid file warnings)

**Files to create:**

- [src/lib/components/FilePicker.svelte](src/lib/components/FilePicker.svelte)
- [src/lib/components/DataTable.svelte](src/lib/components/DataTable.svelte)
- [src/lib/components/MetadataPanel.svelte](src/lib/components/MetadataPanel.svelte)
- [src/lib/stores/parquetStore.ts](src/lib/stores/parquetStore.ts) - Svelte stores for state management

**Files to modify:**

- [src/routes/+page.svelte](src/routes/+page.svelte) - Main application layout

#### 1.3 Data Management

- [ ] Implement pagination for large files
- [ ] Add column type detection and formatting
  - Dates/timestamps
  - Numbers with proper precision
  - Strings with truncation for long values
  - Boolean display
- [ ] Create Svelte stores for:
  - Current file state
  - Table data and pagination
  - Metadata
  - UI preferences (theme, layout)

---

### **Phase 2: Enhanced Viewing Features** 🔍 _Priority: MEDIUM_

#### 2.1 Search and Filter

- [ ] Add search functionality
  - Search across all columns
  - Column-specific search
  - Case-sensitive/insensitive options
- [ ] Implement filtering
  - Filter by column value
  - Range filters for numeric columns
  - Date range filters
  - Multiple filter conditions (AND/OR)
- [ ] Create filter UI component with filter chips

**Files to create:**

- [src/lib/components/SearchBar.svelte](src/lib/components/SearchBar.svelte)
- [src/lib/components/FilterPanel.svelte](src/lib/components/FilterPanel.svelte)

#### 2.2 Data Export

- [ ] Implement export functionality (Rust backend)
  - Export to CSV
  - Export to JSON
  - Export filtered/searched results only
- [ ] Add export UI controls
- [ ] Implement save file dialog

#### 2.3 Column Operations

- [ ] Add column statistics
  - Min/max/avg for numeric columns
  - Unique value count
  - Null count
  - Data distribution preview
- [ ] Column visibility toggle (show/hide columns)
- [ ] Column reordering (drag and drop)
- [ ] Column pinning (freeze left columns)

**Files to create:**

- [src/lib/components/ColumnStats.svelte](src/lib/components/ColumnStats.svelte)
- [src/lib/components/ColumnManager.svelte](src/lib/components/ColumnManager.svelte)

---

### **Phase 3: User Experience Improvements** ✨ _Priority: MEDIUM_

#### 3.1 UI/UX Enhancements

- [ ] Design and implement proper app layout
  - Header with file name and actions
  - Sidebar for metadata/filters (collapsible)
  - Main content area for data table
  - Status bar with row count, file info
- [ ] Add keyboard shortcuts
  - Ctrl+O: Open file
  - Ctrl+F: Search
  - Ctrl+R: Refresh
  - Esc: Clear search/filters
- [ ] Implement dark/light theme toggle
  - Persist theme preference
  - Match system theme by default
- [ ] Add tooltips and help text
- [ ] Implement recent files list
  - Store in Tauri app data directory
  - Quick access to recently opened files

**Files to create:**

- [src/lib/components/AppLayout.svelte](src/lib/components/AppLayout.svelte)
- [src/lib/components/Sidebar.svelte](src/lib/components/Sidebar.svelte)
- [src/lib/components/StatusBar.svelte](src/lib/components/StatusBar.svelte)
- [src/lib/utils/keyboard.ts](src/lib/utils/keyboard.ts)
- [src/lib/stores/themeStore.ts](src/lib/stores/themeStore.ts)

#### 3.2 Performance Optimization

- [ ] Implement virtual scrolling for large datasets
- [ ] Add row virtualization (render only visible rows)
- [ ] Implement column virtualization for wide tables
- [ ] Add memoization for computed values
- [ ] Optimize re-renders with Svelte 5 runes (`$state`, `$derived`, `$effect`)
- [ ] Add web workers for heavy computations (if needed)

#### 3.3 File Management

- [ ] Add file watcher (auto-reload on file change)
- [ ] Implement "Open Folder" to view multiple parquet files
- [ ] Add file comparison mode (view two files side-by-side)
- [ ] Show file path in title bar/status bar

---

### **Phase 4: Advanced Features** 🚀 _Priority: LOW_

#### 4.1 Data Visualization

- [ ] Add chart/graph capabilities
  - Column value distribution (histograms)
  - Time series charts for timestamp columns
  - Correlation matrices
- [ ] Integrate charting library (Chart.js, D3, or Plotly)
- [ ] Create visualization panel

**Files to create:**

- [src/lib/components/ChartPanel.svelte](src/lib/components/ChartPanel.svelte)
- [src/lib/utils/chartHelpers.ts](src/lib/utils/chartHelpers.ts)

#### 4.2 Query and Transformation

- [ ] Add SQL-like query interface
  - Use DuckDB or DataFusion for querying
  - Syntax highlighting for queries
  - Query history
- [ ] Implement data transformations
  - Column derivation
  - Aggregations (GROUP BY)
  - Joins (if multiple files open)
- [ ] Save query results as new parquet file

#### 4.3 Schema and Metadata Tools

- [ ] Schema editor/viewer
  - Visualize nested structures
  - Show schema as tree view
- [ ] Schema comparison (compare two parquet files)
- [ ] Schema export (JSON, DDL)

#### 4.4 Advanced Export

- [ ] Export to other formats
  - Excel (.xlsx)
  - Arrow IPC
  - Avro
  - Feather
- [ ] Batch export (entire folder)
- [ ] Custom export templates

---

### **Phase 5: Polish and Distribution** 📦 _Priority: VARIES_

#### 5.1 Testing

- [ ] Add unit tests for Rust commands
- [ ] Add unit tests for Svelte components (Vitest + Testing Library)
- [ ] Add E2E tests (Playwright or Tauri's testing tools)
- [ ] Test with various parquet files (different schemas, sizes, compressions)
- [ ] Performance benchmarking

**Files to create:**

- [src/lib/components/**tests**/](src/lib/components/__tests__/) - Component tests
- [src-tauri/src/tests/](src-tauri/src/tests/) - Rust unit tests

#### 5.2 Documentation

- [ ] Write user documentation
  - Feature overview
  - Keyboard shortcuts
  - FAQ
- [ ] Create developer documentation
  - Architecture overview
  - Contributing guide
  - Build instructions
- [ ] Add inline code documentation
  - JSDoc for TypeScript
  - Rust doc comments

**Files to create/update:**

- [README.md](README.md) - Project overview and setup
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [docs/USER_GUIDE.md](docs/USER_GUIDE.md)
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)

#### 5.3 Packaging and Distribution

- [ ] Configure Tauri bundler for all platforms
  - Windows: MSI/NSIS installer
  - macOS: DMG and app bundle
  - Linux: AppImage, deb, rpm
- [ ] Set up code signing
  - macOS: Apple Developer certificate
  - Windows: Code signing certificate
- [ ] Create app icons for all platforms
- [ ] Set up auto-updates (Tauri updater)
- [ ] Create release workflow (GitHub Actions)
- [ ] Publish to package managers
  - Windows: winget
  - macOS: Homebrew
  - Linux: Snap, Flatpak

**Files to modify:**

- [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json) - Bundle configuration
- [.github/workflows/release.yml](.github/workflows/release.yml) - CI/CD for releases

#### 5.4 Website and Branding

- [ ] Create landing page
- [ ] Create logo and branding assets
- [ ] Add screenshots and demo GIF
- [ ] Write blog post announcing the app

---

## Technology Recommendations

### Rust Libraries (Backend)

1. **Parquet Reading**:
   - Option A: `arrow` + `parquet` (official Apache Arrow implementation)
   - Option B: `polars` (faster, easier API, more features)
   - **Recommendation**: Start with `polars` for rapid development

2. **File Dialog**: `tauri-plugin-dialog` v2
3. **File System**: `tauri-plugin-fs` v2 (if needed for file watching)
4. **SQL Querying** (Phase 4): `datafusion` or integrate `duckdb`

### Frontend Libraries (Svelte/TypeScript)

1. **Virtual Scrolling**: `svelte-virtual` or `@tanstack/virtual-core`
2. **Data Tables**: Build custom or use `svelte-headless-table`
3. **Charts** (Phase 4): `chart.js` with `svelte-chartjs` or `layerchart`
4. **Icons**: `lucide-svelte` or `svelte-icons`
5. **Styling**:
   - Option A: Tailwind CSS
   - Option B: Custom CSS with CSS variables
   - **Recommendation**: Tailwind for rapid UI development
6. **Testing**: `vitest` + `@testing-library/svelte`

### Development Tools

- **Code Formatting**: Prettier + `prettier-plugin-svelte`
- **Linting**: ESLint for TypeScript, Clippy for Rust
- **Pre-commit Hooks**: Husky + lint-staged
- **Commit Convention**: Conventional Commits

---

## Suggested First Steps

### Immediate Next Steps (Week 1)

1. **Set up Polars in Rust backend**

   ```bash
   # Add to src-tauri/Cargo.toml
   polars = { version = "0.36", features = ["lazy", "parquet"] }
   ```

2. **Implement basic file opening**
   - Add `tauri-plugin-dialog`
   - Create `open_parquet_file` command
   - Test with a sample parquet file

3. **Create minimal UI**
   - File picker button
   - Display column names and row count
   - Show first 100 rows in a simple table

4. **Test the full flow**
   - Open a parquet file
   - Display data in the UI
   - Verify it works end-to-end

### Quick Win Features (Week 2-3)

- Search across columns
- Export to CSV
- Dark mode toggle
- Recent files list
- Column sorting

---

## Project Structure (Proposed)

```
pinkparquet/
├── src/                                    # Frontend (SvelteKit/Svelte)
│   ├── lib/
│   │   ├── components/                     # Svelte components
│   │   │   ├── FilePicker.svelte
│   │   │   ├── DataTable.svelte
│   │   │   ├── MetadataPanel.svelte
│   │   │   ├── SearchBar.svelte
│   │   │   ├── FilterPanel.svelte
│   │   │   ├── ColumnStats.svelte
│   │   │   ├── AppLayout.svelte
│   │   │   ├── Sidebar.svelte
│   │   │   ├── StatusBar.svelte
│   │   │   └── __tests__/                  # Component tests
│   │   ├── stores/                         # Svelte stores
│   │   │   ├── parquetStore.ts
│   │   │   ├── themeStore.ts
│   │   │   └── uiStore.ts
│   │   ├── utils/                          # Utility functions
│   │   │   ├── keyboard.ts
│   │   │   ├── formatters.ts
│   │   │   └── validators.ts
│   │   └── types/                          # TypeScript types
│   │       └── parquet.ts
│   ├── routes/
│   │   ├── +layout.svelte
│   │   ├── +layout.ts
│   │   └── +page.svelte
│   └── app.html
│
├── src-tauri/                              # Backend (Rust/Tauri)
│   ├── src/
│   │   ├── commands/                       # Tauri command modules
│   │   │   ├── mod.rs
│   │   │   ├── file_ops.rs                 # File operations
│   │   │   ├── parquet_reader.rs           # Parquet reading
│   │   │   └── export.rs                   # Export functionality
│   │   ├── utils/                          # Rust utilities
│   │   │   └── mod.rs
│   │   ├── tests/                          # Rust tests
│   │   ├── main.rs
│   │   └── lib.rs
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── icons/
│
├── static/                                 # Static assets
├── docs/                                   # Documentation
│   ├── USER_GUIDE.md
│   └── ARCHITECTURE.md
│
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── release.yml
│
├── tests/                                  # E2E tests
│   └── e2e/
│
├── package.json
├── svelte.config.js
├── vite.config.js
├── tsconfig.json
├── tailwind.config.js                      # If using Tailwind
├── DEVELOPMENT_PLAN.md                     # This file
├── CONTRIBUTING.md
└── README.md
```

---

## Success Metrics

### Phase 1 Success

- ✅ Can open and view any valid parquet file
- ✅ Displays all columns and data types correctly
- ✅ Handles files with 1M+ rows without crashing
- ✅ Metadata panel shows accurate information

### Phase 2 Success

- ✅ Search returns results in < 1 second for most files
- ✅ Filters work correctly for all column types
- ✅ Can export filtered data to CSV

### Phase 3 Success

- ✅ App feels responsive and polished
- ✅ Users can navigate with keyboard shortcuts
- ✅ Recent files list persists between sessions
- ✅ Virtual scrolling handles files with 10M+ rows smoothly

### Phase 4 Success

- ✅ Can generate basic charts from parquet data
- ✅ SQL queries execute and return correct results
- ✅ Schema viewer handles complex nested structures

### Phase 5 Success

- ✅ 80%+ test coverage
- ✅ Installers available for Windows, macOS, Linux
- ✅ Auto-updates work reliably
- ✅ 100+ GitHub stars

---

## Risk Mitigation

### Technical Risks

1. **Performance with very large files**
   - Mitigation: Implement pagination early, use streaming APIs
   - Use benchmarking to identify bottlenecks

2. **Memory usage**
   - Mitigation: Don't load entire file into memory
   - Use Polars' lazy evaluation where possible

3. **Complex nested schemas**
   - Mitigation: Start with flat schemas, add nested support incrementally
   - Test with diverse real-world parquet files

### Development Risks

1. **Scope creep**
   - Mitigation: Stick to phased approach
   - Deliver Phase 1 before adding Phase 4 features

2. **Cross-platform compatibility**
   - Mitigation: Test on all three platforms regularly
   - Use Tauri's CI/CD templates

---

## Resources

### Documentation

- [Tauri Documentation](https://tauri.app/)
- [SvelteKit Documentation](https://kit.svelte.dev/)
- [Svelte 5 Documentation](https://svelte.dev/)
- [Polars Documentation](https://pola.rs/)
- [Apache Arrow Parquet Format](https://parquet.apache.org/)

### Sample Parquet Files for Testing

- [NYC Taxi Dataset](https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page)
- [AWS Public Datasets](https://registry.opendata.aws/)
- [Kaggle Datasets](https://www.kaggle.com/datasets)

---

## Notes

- **Already on Svelte 5.0**: Your app is already using the latest Svelte with modern runes-based reactivity
- **Tauri v2**: Latest version with improved performance and security
- **TypeScript**: Strict mode enabled for better type safety
- **Next Git Init**: Initialize git repository and make initial commit before starting development

---

## Changelog

- **2025-12-13**: Initial development plan created based on codebase exploration
