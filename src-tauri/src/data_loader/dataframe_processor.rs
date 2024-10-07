use std::fs;
use std::path::Path;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{NaiveDateTime, DateTime, Utc};
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
    pub metadata: Option<MetadataInfo>,
}

#[derive(Serialize)]
pub struct MetadataInfo {
    pub file_name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub file_size: u64,
}

#[derive(Deserialize)]
pub struct Sorting {
    pub column: String,
    pub ascending: bool,
}

#[derive(Deserialize)]
pub struct Filtering {
    pub column: String,
    pub condition: String,
    pub value: Value,
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
pub fn process_dataframe(lf: LazyFrame, file_path: &str) -> Result<DataFrameInfo, String> {
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
    let n = shape.0.min(250);
    let df_head = df.head(Some(n));
    let rows = dataframe_to_rows(&df_head);

    let metadata = get_file_metadata(file_path).ok();

    Ok(DataFrameInfo {
        shape,
        columns,
        rows,
        metadata,
    })
}

pub fn get_file_metadata(file_path: &str) -> Result<MetadataInfo, String> {
    match fs::metadata(file_path) {
        Ok(metadata) => {
            let file_name = Path::new(file_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown")
                .to_string();
            let created_at = metadata.created().ok().map(|time| DateTime::<Utc>::from(time));
            let modified_at = metadata.modified().ok().map(|time| DateTime::<Utc>::from(time));
            let file_size = metadata.len(); // Change file_size to direct u64 value

            Ok(MetadataInfo {
                file_name,
                created_at,
                modified_at,
                file_size,
            })
        }
        Err(e) => Err(format!("Failed to get file metadata: {}", e)),
    }
}


pub fn filter_columns(lf: LazyFrame, filtering_info: Vec<Filtering>) -> Result<LazyFrame, String> {
    let mut filtered_lf = lf;

    for filter in filtering_info {
        match filter.condition.as_str() {
            "<" => {
                if let Value::Number(num) = &filter.value {
                    if let Some(f) = num.as_f64() {
                        filtered_lf = filtered_lf.filter(col(&filter.column).lt(lit(f)));
                    } else {
                        return Err("Invalid number value for filter".to_string());
                    }
                } else if let Value::String(date_str) = &filter.value {
                    if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f") {
                        filtered_lf = filtered_lf.filter(col(&filter.column).lt(lit(date)));
                    } else {
                        return Err("Invalid date value for filter".to_string());
                    }
                } else {
                    return Err("Invalid value type for filter".to_string());
                }
            }
            ">" => {
                if let Value::Number(num) = &filter.value {
                    if let Some(f) = num.as_f64() {
                        filtered_lf = filtered_lf.filter(col(&filter.column).gt(lit(f)));
                    } else {
                        return Err("Invalid number value for filter".to_string());
                    }
                } else if let Value::String(date_str) = &filter.value {
                    if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f") {
                        filtered_lf = filtered_lf.filter(col(&filter.column).gt(lit(date)));
                    } else {
                        return Err("Invalid date value for filter".to_string());
                    }
                } else {
                    return Err("Invalid value type for filter".to_string());
                }
            }
            "<=" => {
                if let Value::Number(num) = &filter.value {
                    if let Some(f) = num.as_f64() {
                        filtered_lf = filtered_lf.filter(col(&filter.column).lt_eq(lit(f)));
                    } else {
                        return Err("Invalid number value for filter".to_string());
                    }
                } else if let Value::String(date_str) = &filter.value {
                    if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f") {
                        filtered_lf = filtered_lf.filter(col(&filter.column).lt_eq(lit(date)));
                    } else {
                        return Err("Invalid date value for filter".to_string());
                    }
                } else {
                    return Err("Invalid value type for filter".to_string());
                }
            }
            ">=" => {
                if let Value::Number(num) = &filter.value {
                    if let Some(f) = num.as_f64() {
                        filtered_lf = filtered_lf.filter(col(&filter.column).gt_eq(lit(f)));
                    } else {
                        return Err("Invalid number value for filter".to_string());
                    }
                } else if let Value::String(date_str) = &filter.value {
                    if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f") {
                        filtered_lf = filtered_lf.filter(col(&filter.column).gt_eq(lit(date)));
                    } else {
                        return Err("Invalid date value for filter".to_string());
                    }
                } else {
                    return Err("Invalid value type for filter".to_string());
                }
            }
            "==" => {
                if let Value::Number(num) = &filter.value {
                    if let Some(f) = num.as_f64() {
                        filtered_lf = filtered_lf.filter(col(&filter.column).eq(lit(f)));
                    } else {
                        return Err("Invalid number value for filter".to_string());
                    }
                } else if let Value::String(date_str) = &filter.value {
                    if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f") {
                        filtered_lf = filtered_lf.filter(col(&filter.column).eq(lit(date)));
                    } else {
                        return Err("Invalid date value for filter".to_string());
                    }
                } else {
                    return Err("Invalid value type for equality filter".to_string());
                }
            }
            "!=" => {
                if let Value::Number(num) = &filter.value {
                    if let Some(f) = num.as_f64() {
                        filtered_lf = filtered_lf.filter(col(&filter.column).neq(lit(f)));
                    } else {
                        return Err("Invalid number value for filter".to_string());
                    }
                } else if let Value::String(date_str) = &filter.value {
                    if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f") {
                        filtered_lf = filtered_lf.filter(col(&filter.column).neq(lit(date)));
                    } else {
                        return Err("Invalid date value for filter".to_string());
                    }
                } else {
                    return Err("Invalid value type for equality filter".to_string());
                }
            }
            "between" => {
                if let Value::Array(values) = &filter.value {
                    if values.len() == 2 {
                        if let (Some(lower), Some(upper)) = (values[0].as_f64(), values[1].as_f64()) {
                            filtered_lf = filtered_lf.filter(
                                col(&filter.column)
                                    .gt_eq(lit(lower))
                                    .and(col(&filter.column).lt_eq(lit(upper))),
                            );
                        } else if let (Some(lower), Some(upper)) = (
                            values[0].as_str().and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f").ok()),
                            values[1].as_str().and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f").ok()),
                        ) {
                            filtered_lf = filtered_lf.filter(
                                col(&filter.column)
                                    .gt_eq(lit(lower))
                                    .and(col(&filter.column).lt_eq(lit(upper))),
                            );
                        } else {
                            return Err("Invalid values for between filter".to_string());
                        }
                    } else {
                        return Err("Invalid value count for between filter".to_string());
                    }
                } else {
                    return Err("Invalid value type for between filter".to_string());
                }
            }
            "equals" => {
                if let Value::String(val) = &filter.value {
                    filtered_lf = filtered_lf.filter(col(&filter.column).eq(lit(val.as_str())));
                } else if let Value::String(date_str) = &filter.value {
                    if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f") {
                        filtered_lf = filtered_lf.filter(col(&filter.column).eq(lit(date)));
                    } else {
                        return Err("Invalid date value for equality filter".to_string());
                    }
                } else {
                    return Err("Invalid value type for equality filter".to_string());
                }
            }
            "different" => {
                if let Value::String(val) = &filter.value {
                    filtered_lf = filtered_lf.filter(col(&filter.column).neq(lit(val.as_str())));
                } else if let Value::String(date_str) = &filter.value {
                    if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f") {
                        filtered_lf = filtered_lf.filter(col(&filter.column).neq(lit(date)));
                    } else {
                        return Err("Invalid date value for equality filter".to_string());
                    }
                } else {
                    return Err("Invalid value type for equality filter".to_string());
                }
            }
            _ => return Err(format!("Unknown filter condition: {}", filter.condition)),
        }
    }

    Ok(filtered_lf)
}


