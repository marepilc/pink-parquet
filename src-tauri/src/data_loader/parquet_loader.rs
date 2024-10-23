use polars::prelude::*;

pub fn open_parquet(file_path: &str) -> Result<LazyFrame, String> {
    let args = ScanArgsParquet::default();

    let lf = LazyFrame::scan_parquet(file_path, args)
        .map_err(|e| format!("Failed to read Parquet file: {}", e))?;

    let lf = lf.with_columns([
        dtype_cols(vec![DataType::Float32, DataType::Float64]).fill_nan(lit(NULL)),
    ]);

    Ok(lf)
}

pub fn collect_dataframe(lf: LazyFrame) -> Result<DataFrame, String> {
    let df = match lf.collect() {
        Ok(df) => df,
        Err(e) => return Err(format!("Failed to collect DataFrame: {}", e)),
    };

    Ok(df)
}