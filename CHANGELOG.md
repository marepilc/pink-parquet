# Changelog

All notable changes to this project will be documented in this file.

## [2.0.4] - 2026-01-08

### Added

- **File Watching**:
    - Implemented real-time file watching for opened Parquet files.
    - Added automatic reloading of data when the underlying file is modified on disk.

### Changed

- **Info Bar**:
    - File size information is now context-aware: it is displayed in the table view (representing the file size on
      disk) and hidden in the SQL query view to avoid confusion.

### Fixed

## [2.0.3] - 2026-01-07

### Added

- **Linux Support**:
    - Added Linux build target to the project.
    - Implemented platform-specific title bar handling: custom title bar on Windows, native system title bar on macOS
      and Linux for better OS integration and reliability.

### Changed

- **Window Management**:
    - Refactored title bar implementation to use native system decorations on Linux and macOS, while maintaining custom
      title bar on Windows.
    - Added appropriate padding for macOS and Linux layouts to account for native title bar spacing.

### Fixed

- **Window Management**:
    - Fixed window dragging reliability across all platforms by using platform-appropriate methods (CSS-based on
      Windows, native decorations on Linux/macOS).
    - Added `role="status"` to the footer path container to satisfy accessibility requirements.

## [2.0.2] - 2026-01-04

### Changed

- **SQL Editor**:
    - Simplified automatic capitalization of keywords using CSS `text-transform: uppercase` instead of JavaScript-based
      text replacement.
    - SQL editor now automatically focuses when the component is mounted or becomes visible.

## [2.0.1] - 2026-01-03

### Added

- **Footer Path Display**:
    - Added truncation with ellipsis for long file paths in the status bar.
    - Implemented a smooth sliding transition on hover to reveal the end of long paths (filenames).
    - Added dynamic overflow detection to only enable the transition when the path is too long for the available space.
    - Integrated tooltips to show the full path on hover.

### Fixed

- **Table Tooltips**:
    - Increased tooltip delay to 1000ms for a more relaxed user experience.
    - Improved positioning logic: tooltips now appear near the cursor instead of the cell center.
    - Added smart boundary detection to prevent tooltips from being cut off at the screen edges.
    - Tooltips now immediately hide on scroll or mouse movement to stay out of the way.
- **SQL Editor**:
    - Fixed over-aggressive auto-capitalization: table and column names matching SQL keywords (e.g., `date`, `count`)
      are no longer capitalized if they exist in the schema.
    - Improved configuration reactivity for autocompletion and keyword rules.
- **Data Table Sorting**:
    - Fixed a bug where sorting indicators remained active after closing or switching files.
    - Refactored sorting state to be session-aware, ensuring it persists when switching tabs but resets correctly for
      new files.
    - Optimized performance: applying a sort now returns the view to the first 100 rows, preventing UI lag with large
      datasets.
    - Fixed horizontal scroll jumping: the table now maintains its horizontal position when sorting or switching between
      Raw and SQL views.
