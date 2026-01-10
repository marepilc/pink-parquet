use polars::prelude::*;

/// Opens a CSV file and returns a LazyFrame
pub fn open_csv(file_path: &str) -> Result<LazyFrame, String> {
    LazyCsvReader::new(PlPath::from_str(file_path))
        .with_infer_schema_length(Some(10000))
        .with_has_header(true)
        .finish()
        .map_err(|e| format!("Failed to scan CSV file: {}", e))
}
