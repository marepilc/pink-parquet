use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Sorting {
    pub column: String,
    pub ascending: bool,
}

/// Apply sorts to a LazyFrame
pub fn apply_sorts(lf: LazyFrame, sorts: Vec<Sorting>) -> Result<LazyFrame, String> {
    if sorts.is_empty() {
        return Ok(lf);
    }

    let columns: Vec<String> = sorts.iter().map(|s| s.column.clone()).collect();
    let descending: Vec<bool> = sorts.iter().map(|s| !s.ascending).collect();

    let options = SortMultipleOptions::default().with_order_descending_multi(descending);

    let result = lf.sort(columns, options);

    Ok(result)
}

/// Convert DataFrame to rows (Vec<Vec<String>>)
pub fn dataframe_to_rows(df: &DataFrame) -> Result<Vec<Vec<String>>, String> {
    let height = df.height();
    let mut rows = vec![vec![String::new(); df.width()]; height];

    for (col_idx, series) in df.get_columns().iter().enumerate() {
        // Handle different data types appropriately
        match series.dtype() {
            DataType::Binary => {
                // Binary data - convert to hex string representation
                if let Ok(ca) = series.binary() {
                    for (row_idx, opt_val) in ca.into_iter().enumerate() {
                        rows[row_idx][col_idx] = if let Some(val) = opt_val {
                            format!("0x{}", hex::encode(val))
                        } else {
                            String::new()
                        };
                    }
                }
            }
            DataType::List(_) => {
                // List types - format as [item1, item2, ...]
                for row_idx in 0..height {
                    let value = series.get(row_idx).unwrap();
                    rows[row_idx][col_idx] = format_list_value(&value);
                }
            }
            DataType::Array(_, _) => {
                // Array types (fixed-size lists) - format as [item1, item2, ...]
                for row_idx in 0..height {
                    let value = series.get(row_idx).unwrap();
                    rows[row_idx][col_idx] = format_list_value(&value);
                }
            }
            DataType::Struct(_) => {
                // Struct types - format as {field1: value1, field2: value2, ...}
                for row_idx in 0..height {
                    let value = series.get(row_idx).unwrap();
                    rows[row_idx][col_idx] = format_struct_value(&value);
                }
            }
            DataType::Duration(time_unit) => {
                // Duration types - format as human-readable duration
                for row_idx in 0..height {
                    let value = series.get(row_idx).unwrap();
                    rows[row_idx][col_idx] = if value.is_null() {
                        String::new()
                    } else if let Ok(val) = value.try_extract::<i64>() {
                        format_duration(val, time_unit)
                    } else {
                        format!("{:?}", value)
                    };
                }
            }
            _ => {
                // For all other types, try to cast to String
                match series.cast(&DataType::String) {
                    Ok(s_str) => {
                        if let Ok(ca) = s_str.str() {
                            for (row_idx, opt_val) in ca.into_iter().enumerate() {
                                if let Some(val) = opt_val {
                                    rows[row_idx][col_idx] = val.to_string();
                                } else {
                                    rows[row_idx][col_idx] = String::new();
                                }
                            }
                        } else {
                            // Fallback to debug format if str() fails
                            for row_idx in 0..height {
                                rows[row_idx][col_idx] = format!("{:?}", series.get(row_idx).unwrap());
                            }
                        }
                    }
                    Err(_) => {
                        // If cast fails, use debug format as fallback
                        for row_idx in 0..height {
                            rows[row_idx][col_idx] = format!("{:?}", series.get(row_idx).unwrap());
                        }
                    }
                }
            }
        }
    }

    Ok(rows)
}

/// Calculate statistics for all columns in a DataFrame
pub fn calculate_statistics(
    df: &DataFrame,
) -> Result<HashMap<String, HashMap<String, serde_json::Value>>, String> {
    let mut stats: HashMap<String, HashMap<String, serde_json::Value>> = HashMap::new();

    for series in df.get_columns() {
        let column_name = series.name().to_string();
        let mut column_stats: HashMap<String, serde_json::Value> = HashMap::new();

        // Null count (applicable to all types)
        let null_count = series.null_count();
        column_stats.insert("null_values".to_string(), serde_json::json!(null_count));

        match series.dtype() {
            DataType::Boolean => {
                // Boolean statistics - count true/false values
                if let Ok(ca) = series.bool() {
                    let total = series.len() - null_count;
                    let true_count = ca.sum().unwrap_or(0) as usize;
                    let false_count = total - true_count;

                    let true_pct = if total > 0 {
                        (true_count as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    };
                    let false_pct = if total > 0 {
                        (false_count as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    };

                    column_stats.insert(
                        "true_count".to_string(),
                        serde_json::json!(format!("{} ({:.1}%)", true_count, true_pct))
                    );
                    column_stats.insert(
                        "false_count".to_string(),
                        serde_json::json!(format!("{} ({:.1}%)", false_count, false_pct))
                    );
                }
            }
            DataType::Int8
            | DataType::Int16
            | DataType::Int32
            | DataType::Int64
            | DataType::UInt8
            | DataType::UInt16
            | DataType::UInt32
            | DataType::UInt64
            | DataType::Float32
            | DataType::Float64 => {
                // Numeric statistics
                if let Ok(ca) = series.cast(&DataType::Float64) {
                    if let Ok(series_f64) = ca.f64() {
                        if let Some(min) = series_f64.min() {
                            column_stats.insert("min".to_string(), serde_json::json!(min));
                        }
                        if let Some(max) = series_f64.max() {
                            column_stats.insert("max".to_string(), serde_json::json!(max));
                        }
                        if let Some(mean) = series_f64.mean() {
                            column_stats.insert("mean".to_string(), serde_json::json!(mean));
                        }
                        if let Some(median) = series_f64.median() {
                            column_stats.insert("median".to_string(), serde_json::json!(median));
                        }
                        if let Some(q25) = series_f64
                            .quantile(0.25, QuantileMethod::Linear)
                            .ok()
                            .flatten()
                        {
                            column_stats
                                .insert("percentile_25".to_string(), serde_json::json!(q25));
                        }
                        if let Some(q75) = series_f64
                            .quantile(0.75, QuantileMethod::Linear)
                            .ok()
                            .flatten()
                        {
                            column_stats
                                .insert("percentile_75".to_string(), serde_json::json!(q75));
                        }
                    }
                }
            }
            DataType::Date | DataType::Datetime(_, _) | DataType::Time | DataType::Duration(_) => {
                // Date/time statistics (min, max) - simplified for now
                // These can be improved with proper datetime handling
            }
            _ => {
                // String and other types - unique count
                if let Ok(unique_count) = series.unique().map(|u| u.len()) {
                    column_stats
                        .insert("unique_values".to_string(), serde_json::json!(unique_count));
                }
            }
        }

        stats.insert(column_name, column_stats);
    }

    Ok(stats)
}

/// Format duration value as human-readable string (e.g., "21 days 12:00:00")
fn format_duration(value: i64, time_unit: &polars::prelude::TimeUnit) -> String {
    // Convert to microseconds for consistent handling
    let microseconds = match time_unit {
        polars::prelude::TimeUnit::Nanoseconds => value / 1_000,
        polars::prelude::TimeUnit::Microseconds => value,
        polars::prelude::TimeUnit::Milliseconds => value * 1_000,
    };

    let is_negative = microseconds < 0;
    let abs_microseconds = microseconds.abs();

    // Calculate components
    let total_seconds = abs_microseconds / 1_000_000;
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    let sign = if is_negative { "-" } else { "" };

    if days > 0 {
        format!("{}{} days {:02}:{:02}:{:02}", sign, days, hours, minutes, seconds)
    } else {
        format!("{}{:02}:{:02}:{:02}", sign, hours, minutes, seconds)
    }
}

/// Format List/Array value as simple array notation (e.g., "[1, 2, 3]")
fn format_list_value(value: &polars::prelude::AnyValue) -> String {
    use polars::prelude::AnyValue;

    match value {
        AnyValue::List(series) => {
            let items: Vec<String> = (0..series.len())
                .map(|i| {
                    let item = series.get(i).unwrap();
                    format_any_value(&item)
                })
                .collect();
            format!("[{}]", items.join(", "))
        }
        // Handle Array type (fixed-size list) - extract from debug format or convert
        _ => {
            // Check if it's an array by converting to string and parsing
            let debug_str = format!("{:?}", value);
            if debug_str.starts_with("Array(") {
                // Extract just the values from the Array debug format
                // Try to extract the series and format it
                if let AnyValue::Null = value {
                    String::new()
                } else {
                    // For arrays, we'll use a simpler approach - just show the type
                    // since extracting values from Array AnyValue is complex
                    debug_str
                        .split("Series: '' [")
                        .nth(1)
                        .and_then(|s| s.split(']').next())
                        .map(|type_str| {
                            // Try to extract the values
                            if let Some(values_part) = debug_str.split("[\n").nth(1) {
                                let values: Vec<String> = values_part
                                    .lines()
                                    .filter_map(|line| {
                                        let trimmed = line.trim();
                                        if !trimmed.is_empty() && trimmed != "]" && !trimmed.starts_with("],") {
                                            Some(trimmed.to_string())
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                format!("[{}]", values.join(", "))
                            } else {
                                format!("array<{}>", type_str)
                            }
                        })
                        .unwrap_or_else(|| debug_str)
                }
            } else if let AnyValue::Null = value {
                String::new()
            } else {
                format!("{:?}", value)
            }
        }
    }
}

/// Format Struct value as object notation (e.g., "{x: 42, y: 3.14}")
fn format_struct_value(value: &polars::prelude::AnyValue) -> String {
    use polars::prelude::AnyValue;

    match value {
        AnyValue::Struct(idx, arr, fields) => {
            let mut field_strs = Vec::new();
            let values = arr.values();
            for (field_idx, field) in fields.iter().enumerate() {
                if field_idx < values.len() {
                    // Convert Arrow array to Polars Series for easier value extraction
                    let arrow_array = &values[field_idx];
                    if let Ok(series) = polars::prelude::Series::try_from((field.name().clone(), arrow_array.clone())) {
                        if let Ok(field_value) = series.get(*idx) {
                            field_strs.push(format!("{}: {}", field.name(), format_any_value_simple(&field_value)));
                        }
                    }
                }
            }
            format!("{{{}}}", field_strs.join(", "))
        }
        AnyValue::StructOwned(_) => {
            // For StructOwned, use simplified format
            format!("{:?}", value)
        }
        AnyValue::Null => String::new(),
        _ => format!("{:?}", value)
    }
}

/// Simplified formatting for nested values (avoids recursion issues)
fn format_any_value_simple(value: &polars::prelude::AnyValue) -> String {
    use polars::prelude::AnyValue;

    match value {
        AnyValue::Null => "null".to_string(),
        AnyValue::Boolean(b) => b.to_string(),
        AnyValue::String(s) => s.to_string(),
        AnyValue::StringOwned(s) => s.to_string(),
        AnyValue::Int8(v) => v.to_string(),
        AnyValue::Int16(v) => v.to_string(),
        AnyValue::Int32(v) => v.to_string(),
        AnyValue::Int64(v) => v.to_string(),
        AnyValue::UInt8(v) => v.to_string(),
        AnyValue::UInt16(v) => v.to_string(),
        AnyValue::UInt32(v) => v.to_string(),
        AnyValue::UInt64(v) => v.to_string(),
        AnyValue::Float32(v) => format!("{:.6}", v),
        AnyValue::Float64(v) => format!("{:.6}", v),
        _ => format!("{:?}", value)
    }
}

/// Format a single AnyValue for display in lists/structs
fn format_any_value(value: &polars::prelude::AnyValue) -> String {
    use polars::prelude::AnyValue;

    match value {
        AnyValue::Null => "null".to_string(),
        AnyValue::Boolean(b) => b.to_string(),
        AnyValue::String(s) => format!("\"{}\"", s),
        AnyValue::StringOwned(s) => format!("\"{}\"", s),
        AnyValue::Binary(_) | AnyValue::BinaryOwned(_) => "binary".to_string(),
        AnyValue::Int8(v) => v.to_string(),
        AnyValue::Int16(v) => v.to_string(),
        AnyValue::Int32(v) => v.to_string(),
        AnyValue::Int64(v) => v.to_string(),
        AnyValue::UInt8(v) => v.to_string(),
        AnyValue::UInt16(v) => v.to_string(),
        AnyValue::UInt32(v) => v.to_string(),
        AnyValue::UInt64(v) => v.to_string(),
        AnyValue::Float32(v) => v.to_string(),
        AnyValue::Float64(v) => v.to_string(),
        AnyValue::List(series) => {
            let items: Vec<String> = (0..series.len().min(3))
                .map(|i| format_any_value(&series.get(i).unwrap()))
                .collect();
            if series.len() > 3 {
                format!("[{}, ...]", items.join(", "))
            } else {
                format!("[{}]", items.join(", "))
            }
        }
        AnyValue::Struct(_, _, _) => {
            // For nested structs in lists, use simplified format
            "struct".to_string()
        }
        _ => format!("{:?}", value)
    }
}
