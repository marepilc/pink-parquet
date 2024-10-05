mod data_loader;

use tauri::command;
use data_loader::{open_parquet, process_dataframe, DataFrameInfo, dataframe_to_rows, collect_dataframe_safe};
use data_loader::dataframe_processor::{sort_columns, Sorting};
use std::collections::HashMap;

#[command]
fn get_data(file_path: &str, sorting: Option<Vec<Sorting>>) -> Result<DataFrameInfo, String> {
    let lf = match open_parquet(file_path) {
        Ok(df) => df,
        Err(e) => return Err(format!("Failed to read Parquet file: {}", e)),
    };

    if let Some(sorting_info) = sorting {
        let df = match sort_columns(lf, sorting_info) {
            Ok(df) => df,
            Err(e) => return Err(format!("Failed to sort DataFrame: {}", e)),
        };
        let df_info = match process_dataframe(df, file_path) {
            Ok(df_info) => df_info,
            Err(e) => return Err(format!("Failed to process DataFrame: {}", e)),
        };
        return Ok(df_info);
    }

    let df_info = match process_dataframe(lf, file_path) {
        Ok(df_info) => df_info,
        Err(e) => return Err(format!("Failed to process DataFrame: {}", e)),
    };

    Ok(df_info)  // Tauri will automatically serialize this to JSON
}

#[command]
fn get_more_rows(file_path: &str, offset: usize, limit: usize, sorting: Option<Vec<Sorting>>) -> Result<Vec<Vec<String>>, String> {
    let lf = match open_parquet(file_path) {
        Ok(df) => df,
        Err(e) => return Err(format!("Failed to read Parquet file: {}", e)),
    };
    let sorted_lf = if let Some(sorting_info) = sorting {
        match sort_columns(lf, sorting_info) {
            Ok(df) => df,
            Err(e) => return Err(format!("Failed to sort DataFrame: {}", e)),
        }
    } else {
        lf // No sorting, just use the original DataFrame
    };
    let df = collect_dataframe_safe(sorted_lf)?;
    let df_slice = df.slice(offset as i64, limit);
    let rows = dataframe_to_rows(&df_slice);
    Ok(rows)
}

#[command]
fn get_statistics(file_path: &str) -> Result<HashMap<String, HashMap<String, f64>>, String> {
    // Open the Parquet file
    let lf = match open_parquet(file_path) {
        Ok(lf) => lf,
        Err(e) => return Err(format!("Failed to read Parquet file: {}", e)),
    };

    // Collect the DataFrame safely
    let df = collect_dataframe_safe(lf)?;

    let mut statistics = HashMap::new();

    for column in df.get_columns() {
        let column_name = column.name();
        let mut column_stats = HashMap::new();

        // Check if column is numeric or date
        if column.dtype().is_numeric() || column.dtype() == &polars::prelude::DataType::Date || column.dtype() == &polars::prelude::DataType::Datetime(polars::prelude::TimeUnit::Milliseconds, None) {
            // Null values count
            let null_values = column.null_count() as f64;
            column_stats.insert("null_values".to_string(), null_values);

            // Min, Max, Mean, Median, 25th and 75th Percentile
            if let Ok(Some(min)) = column.min() {
                column_stats.insert("min".to_string(), min);
            }

            if let Ok(Some(max)) = column.max() {
                column_stats.insert("max".to_string(), max);
            }

            if let Some(mean) = column.mean() {
                column_stats.insert("mean".to_string(), mean);
            }

            if let Some(median) = column.median() {
                column_stats.insert("median".to_string(), median);
            }

            // Replace quantile method with manual percentile calculation
            let sort_options = polars::prelude::SortOptions {
                descending: false,
                nulls_last: false,
                ..Default::default()
            };

            if let Ok(sorted_column) = column.sort(sort_options) {
                let len = sorted_column.len();
                if len > 0 {
                    let p25_idx = (0.25 * (len - 1) as f64).round() as usize;
                    let p75_idx = (0.75 * (len - 1) as f64).round() as usize;

                    if let Ok(p25_value) = sorted_column.get(p25_idx) {
                        match p25_value {
                            polars::prelude::AnyValue::Float64(val) => {
                                column_stats.insert("percentile_25".to_string(), val);
                            }
                            polars::prelude::AnyValue::Float32(val) => {
                                column_stats.insert("percentile_25".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::Datetime(val, _, _) => {
                                column_stats.insert("percentile_25".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::Int64(val) => {
                                column_stats.insert("percentile_25".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::UInt32(val) => {
                                column_stats.insert("percentile_25".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::UInt64(val) => {
                                column_stats.insert("percentile_25".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::Int32(val) => {
                                column_stats.insert("percentile_25".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::UInt16(val) => {
                                column_stats.insert("percentile_25".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::Int16(val) => {
                                column_stats.insert("percentile_25".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::UInt8(val) => {
                                column_stats.insert("percentile_25".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::Int8(val) => {
                                column_stats.insert("percentile_25".to_string(), val as f64);
                            }
                            _ => {}
                        }
                    }

                    if let Ok(p75_value) = sorted_column.get(p75_idx) {
                        match p75_value {
                            polars::prelude::AnyValue::Float64(val) => {
                                column_stats.insert("percentile_75".to_string(), val);
                            }
                            polars::prelude::AnyValue::Float32(val) => {
                                column_stats.insert("percentile_75".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::Datetime(val, _, _) => {
                                column_stats.insert("percentile_75".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::Int64(val) => {
                                column_stats.insert("percentile_75".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::UInt32(val) => {
                                column_stats.insert("percentile_75".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::UInt64(val) => {
                                column_stats.insert("percentile_75".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::Int32(val) => {
                                column_stats.insert("percentile_75".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::UInt16(val) => {
                                column_stats.insert("percentile_75".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::Int16(val) => {
                                column_stats.insert("percentile_75".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::UInt8(val) => {
                                column_stats.insert("percentile_75".to_string(), val as f64);
                            }
                            polars::prelude::AnyValue::Int8(val) => {
                                column_stats.insert("percentile_75".to_string(), val as f64);
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else {
            // Non-numeric column
            // Unique values count (distinct count)
            let unique_values = match column.n_unique() {
                Ok(val) => val as f64, // Convert usize to f64
                Err(e) => return Err(format!("Failed to compute unique values for column '{}': {}", column_name, e)),
            };

            column_stats.insert("unique_values".to_string(), unique_values);

            // Null values count
            let null_values = column.null_count() as f64;
            column_stats.insert("null_values".to_string(), null_values);

            // Min and Max for categorical data
            if let Ok(Some(min)) = column.min() {
                column_stats.insert("min".to_string(), min);
            }

            if let Ok(Some(max)) = column.max() {
                column_stats.insert("max".to_string(), max);
            }
        }

        statistics.insert(column_name.to_string(), column_stats);
    }

    Ok(statistics)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_data, get_more_rows, get_statistics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
