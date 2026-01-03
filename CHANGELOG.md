# Changelog

All notable changes to this project will be documented in this file.

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
