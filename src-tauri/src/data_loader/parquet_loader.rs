use polars::prelude::*;

pub fn open_parquet(file_path: &str) -> Result<LazyFrame, String> {
    let args = ScanArgsParquet::default();

    let lf = match LazyFrame::scan_parquet(file_path, args) {
        Ok(lf) => lf,
        Err(e) => return Err(format!("Failed to read Parquet file: {}", e)),
    };

    Ok(lf)
}

pub fn collect_dataframe(lf: LazyFrame) -> Result<DataFrame, String> {
    let df = match lf.collect() {
        Ok(df) => df,
        Err(e) => return Err(format!("Failed to collect DataFrame: {}", e)),
    };

    Ok(df)
}