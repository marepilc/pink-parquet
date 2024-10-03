use polars::prelude::*;
use serde::{Deserialize, Serialize};
use crate::data_loader::collect_dataframe;

#[derive(Serialize)]
pub struct ColumnInfo {
    pub name: String,
    pub dtype: String,
}

#[derive(Serialize)]
pub struct DataFrameInfo {
    pub shape: (usize, usize),
    pub columns: Vec<ColumnInfo>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Sorting {
    pub column: String,
    pub ascending: bool,
}

pub fn collect_dataframe_safe(lf: LazyFrame) -> Result<DataFrame, String> {
    collect_dataframe(lf).map_err(|e| format!("Failed to collect DataFrame: {}", e))
}

pub fn sort_columns(lf: LazyFrame, sorting_info: Vec<Sorting>) -> Result<LazyFrame, String> {
    let column_names: Vec<PlSmallStr> = sorting_info
        .iter()
        .map(|s| PlSmallStr::from(s.column.as_str()))
        .collect();
    let orders: Vec<bool> = sorting_info.iter().map(|s| s.ascending).collect();
    let nulls_last = vec![true; column_names.len()];
    let sorted = lf.sort(
        column_names,
        SortMultipleOptions {
            descending: orders.iter().map(|&asc| !asc).collect(), // Convert ascending to descending
            multithreaded: true,
            nulls_last,
            ..Default::default()
        },
    );
    Ok(sorted)
}

fn anyvalue_to_string(value: AnyValue) -> String {
    match value {
        AnyValue::String(s) => s.to_string(),
        AnyValue::Null => "".to_string(),
        AnyValue::Categorical(ix, rev_mapping, ..) => {
            rev_mapping.get(ix).to_string()
        }
        _ => value.to_string(),
    }
}

pub fn dataframe_to_rows(df: &DataFrame) -> Vec<Vec<String>> {
    (0..df.height())
        .map(|row_idx| {
            df.get_columns()
                .iter()
                .map(|series| {
                    series.get(row_idx).map_or_else(
                        |_| "".to_string(),
                        |v| anyvalue_to_string(v),
                    )
                })
                .collect()
        })
        .collect()
}


// Function to process the DataFrame and return its info
pub fn process_dataframe(lf: LazyFrame) -> Result<DataFrameInfo, String> {
    let df = collect_dataframe_safe(lf)?;
    let shape = df.shape();
    let columns: Vec<ColumnInfo> = df
        .get_columns()
        .iter()
        .map(|col| ColumnInfo {
            name: col.name().to_string(),
            dtype: format!("{:?}", col.dtype()),
        })
        .collect();

    // Get the first n rows (for example, the first 100 rows)
    let n = shape.0.min(100);
    let df_head = df.head(Some(n));

    let rows = dataframe_to_rows(&df_head);

    Ok(DataFrameInfo {
        shape,
        columns,
        rows,
    })
}
