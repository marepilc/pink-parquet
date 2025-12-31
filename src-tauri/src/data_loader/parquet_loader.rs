use polars::prelude::*;
use std::fs::File;
use parquet::file::reader::{FileReader, SerializedFileReader};

/// Opens a Parquet file and returns a LazyFrame
pub fn open_parquet(file_path: &str) -> Result<LazyFrame, String> {
    // Pre-check the schema for unsupported types
    check_schema_compatibility(file_path)?;

    LazyFrame::scan_parquet(PlPath::from_str(file_path), Default::default())
        .map_err(|e| format!("Failed to scan Parquet file: {}", e))
}

/// Check if the Parquet file schema contains unsupported types
fn check_schema_compatibility(file_path: &str) -> Result<(), String> {
    let file = File::open(file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;

    let reader = SerializedFileReader::new(file)
        .map_err(|e| format!("Failed to read Parquet metadata: {}", e))?;

    let schema = reader.metadata().file_metadata().schema_descr();

    // Check for FixedSizeList types
    for field in schema.columns() {
        let type_str = format!("{:?}", field.physical_type());
        let converted_type_str = format!("{:?}", field.converted_type());

        if converted_type_str.contains("LIST") && type_str.contains("Fixed") {
            return Err(format!(
                "Column '{}' contains Array/FixedSizeList type which is not fully supported. \
                Please convert Array columns to List (variable-size) in your data generation script. \
                Remove the .list.to_array() conversion or use regular lists instead of arrays.",
                field.name()
            ));
        }
    }

    Ok(())
}
