use std::fs::{self, File};
use std::path::Path;
use polars::prelude::*;
use polars::datatypes::DataType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{NaiveDateTime, DateTime, Utc};
use parquet::file::reader::{FileReader};
use parquet::file::serialized_reader::SerializedFileReader;
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
pub struct ColumnMetaData {
    pub name: String,
    pub compression: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataInfo {
    pub file_name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub file_size: u64,
    pub row_groups: usize,
    pub num_rows: usize,
    pub columns: Vec<ColumnMetaData>,
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
        AnyValue::Enum(ix, rev_mapping, ..) => {
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
    // Collect the DataFrame safely from the LazyFrame
    let df = collect_dataframe_safe(lf)?;
    let shape = df.shape();

    // Get column information
    let columns: Vec<ColumnInfo> = df
        .get_columns()
        .iter()
        .map(|col| ColumnInfo {
            name: col.name().to_string(),
            dtype: format!("{:?}", col.dtype()),
        })
        .collect();

    // Get the first n rows (for example, the first 250 rows)
    let n = shape.0.min(250);
    let df_head = df.head(Some(n));
    let rows = dataframe_to_rows(&df_head);

    // Extract metadata from the file
    let metadata = get_file_metadata(file_path).ok();

    Ok(DataFrameInfo {
        shape,
        columns,
        rows,
        metadata,
    })
}


pub fn get_file_metadata(file_path: &str) -> Result<MetadataInfo, String> {
    // Get file system metadata
    let metadata = fs::metadata(file_path).map_err(|e| format!("Failed to get file metadata: {}", e))?;
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown")
        .to_string();
    let created_at = metadata.created().ok().map(|time| DateTime::<Utc>::from(time));
    let modified_at = metadata.modified().ok().map(|time| DateTime::<Utc>::from(time));
    let file_size = metadata.len();

    // Open the Parquet file and read its metadata
    let file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let parquet_reader = SerializedFileReader::new(file).map_err(|e| format!("Failed to read Parquet file: {}", e))?;
    let parquet_metadata = parquet_reader.metadata();

    // Collect row group and column metadata
    let num_row_groups = parquet_metadata.num_row_groups();
    let num_rows = parquet_metadata.file_metadata().num_rows() as usize;
    let mut columns = Vec::new();

    for row_group_idx in 0..num_row_groups {
        let row_group = parquet_metadata.row_group(row_group_idx);
        for column_chunk in row_group.columns() {
            let column_name = column_chunk.column_descr().name().to_string();
            let compression = column_chunk.compression().to_string();
            columns.push(ColumnMetaData {
                name: column_name,
                compression,
            });
        }
    }

    Ok(MetadataInfo {
        file_name,
        created_at,
        modified_at,
        file_size,
        row_groups: num_row_groups,
        num_rows,
        columns,
    })
}


pub fn filter_columns(mut filtered_lf: LazyFrame, filtering_info: Vec<Filtering>) -> Result<LazyFrame, String> {
    for filter in filtering_info {
        match filter.condition.as_str() {
            "<" | ">" | "<=" | ">=" | "==" | "!=" => {
                if let Value::Number(num) = &filter.value {
                    if let Some(f) = num.as_f64() {
                        let column_expr = col(&filter.column);
                        filtered_lf = match filter.condition.as_str() {
                            "<" => filtered_lf.filter(column_expr.lt(lit(f))),
                            ">" => filtered_lf.filter(column_expr.gt(lit(f))),
                            "<=" => filtered_lf.filter(column_expr.lt_eq(lit(f))),
                            ">=" => filtered_lf.filter(column_expr.gt_eq(lit(f))),
                            "==" => filtered_lf.filter(column_expr.eq(lit(f))),
                            "!=" => filtered_lf.filter(column_expr.neq(lit(f))),
                            _ => return Err(format!("Invalid number filter condition: {}", filter.condition)),
                        };
                    } else {
                        return Err("Invalid number value for filter".to_string());
                    }
                } else if let Value::String(date_str) = &filter.value {
                    let datetime_result = DateTime::parse_from_rfc3339(date_str)
                        .map(|dt| dt.with_timezone(&Utc).naive_utc())
                        .or_else(|_| NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f"));

                    if let Ok(date) = datetime_result {
                        let column_expr = col(&filter.column).cast(DataType::Datetime(TimeUnit::Microseconds, Some("UTC".into())));
                        let date_lit = lit(date).cast(DataType::Datetime(TimeUnit::Microseconds, Some("UTC".into())));

                        filtered_lf = match filter.condition.as_str() {
                            "<" => filtered_lf.filter(column_expr.lt(date_lit)),
                            ">" => filtered_lf.filter(column_expr.gt(date_lit)),
                            "<=" => filtered_lf.filter(column_expr.lt_eq(date_lit)),
                            ">=" => filtered_lf.filter(column_expr.gt_eq(date_lit)),
                            "==" => filtered_lf.filter(column_expr.eq(date_lit)),
                            "!=" => filtered_lf.filter(column_expr.neq(date_lit)),
                            _ => return Err(format!("Invalid date filter condition: {}", filter.condition)),
                        };
                    } else {
                        return Err("Invalid date value for filter".to_string());
                    }
                } else if let Value::Bool(val) = &filter.value {
                    filtered_lf = match filter.condition.as_str() {
                        "==" => filtered_lf.filter(col(&filter.column).eq(lit(*val))),
                        "!=" => filtered_lf.filter(col(&filter.column).neq(lit(*val))),
                        _ => return Err(format!("Invalid boolean filter condition: {}", filter.condition)),
                    };
                } else {
                    return Err("Invalid value type for filter".to_string());
                }
            }
            "between" => {
                if let Value::Array(values) = &filter.value {
                    if values.len() == 2 {
                        let lower = &values[0];
                        let upper = &values[1];
                        let column_expr = col(&filter.column);

                        if let (Some(lower), Some(upper)) = (lower.as_f64(), upper.as_f64()) {
                            filtered_lf = filtered_lf.filter(
                                column_expr.clone().gt_eq(lit(lower)).and(column_expr.lt_eq(lit(upper))),
                            );
                        } else if let (Some(lower), Some(upper)) = (
                            lower.as_str().and_then(|s| {
                                DateTime::parse_from_rfc3339(s)
                                    .map(|dt| dt.with_timezone(&Utc).naive_utc())
                                    .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f"))
                                    .ok()
                            }),
                            upper.as_str().and_then(|s| {
                                DateTime::parse_from_rfc3339(s)
                                    .map(|dt| dt.with_timezone(&Utc).naive_utc())
                                    .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f"))
                                    .ok()
                            }),
                        ) {
                            let column_expr = col(&filter.column).cast(DataType::Datetime(TimeUnit::Microseconds, Some("UTC".into())));
                            let lower_lit = lit(lower).cast(DataType::Datetime(TimeUnit::Microseconds, Some("UTC".into())));
                            let upper_lit = lit(upper).cast(DataType::Datetime(TimeUnit::Microseconds, Some("UTC".into())));

                            filtered_lf = filtered_lf.filter(
                                column_expr.clone().gt_eq(lower_lit).and(column_expr.lt_eq(upper_lit)),
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
                } else if let Value::Bool(val) = &filter.value {
                    filtered_lf = filtered_lf.filter(col(&filter.column).eq(lit(*val)));
                } else {
                    return Err("Invalid value type for equality filter".to_string());
                }
            }
            "different" => {
                if let Value::String(val) = &filter.value {
                    filtered_lf = filtered_lf.filter(col(&filter.column).neq(lit(val.as_str())));
                } else if let Value::Bool(val) = &filter.value {
                    filtered_lf = filtered_lf.filter(col(&filter.column).neq(lit(*val)));
                } else {
                    return Err("Invalid value type for inequality filter".to_string());
                }
            }
            "contains_case_insensitive" => {
                if let Value::String(val) = &filter.value {
                    // Create a regex pattern that ignores case
                    let pattern = format!("(?i){}", regex::escape(val));
                    filtered_lf = filtered_lf.filter(
                        col(&filter.column)
                            .cast(DataType::String)
                            .str()
                            .contains(lit(pattern), false),
                    );
                } else {
                    return Err("Invalid value type for case-insensitive contains filter".to_string());
                }
            }
            "contains" => {
                if let Value::String(val) = &filter.value {
                    filtered_lf = filtered_lf.filter(
                        col(&filter.column)
                            .cast(DataType::String)
                            .str()
                            .contains(lit(val.as_str()), false),
                    );
                } else {
                    return Err("Invalid value type for contains filter".to_string());
                }
            }
            "is_null" => {
                filtered_lf = filtered_lf.filter(col(&filter.column).is_null());
            }
            "is_not_null" => {
                filtered_lf = filtered_lf.filter(col(&filter.column).is_not_null());
            }
            _ => return Err(format!("Unknown filter condition: {}", filter.condition)),
        }
    }

    Ok(filtered_lf)
}
