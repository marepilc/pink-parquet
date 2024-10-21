mod data_loader;

use tauri::command;
use data_loader::{open_parquet, process_dataframe, DataFrameInfo, dataframe_to_rows, collect_dataframe_safe};
use data_loader::dataframe_processor::{sort_columns, Sorting, filter_columns, Filtering};
use std::collections::HashMap;

#[command]
fn get_data(file_path: &str, sorting: Option<Vec<Sorting>>, filtering: Option<Vec<Filtering>>) -> Result<DataFrameInfo, String> {
    let mut lf = match open_parquet(file_path) {
        Ok(df) => df,
        Err(e) => return Err(format!("Failed to read Parquet file: {}", e)),
    };

    // Get shape before filtering
    let df = collect_dataframe_safe(lf.clone())?;
    let shape = df.shape();

    if let Some(filtering_info) = filtering {
        lf = match filter_columns(lf, filtering_info) {
            Ok(df) => df,
            Err(e) => return Err(format!("Failed to filter DataFrame: {}", e)),
        };
    }

    if let Some(sorting_info) = sorting {
        lf = match sort_columns(lf, sorting_info) {
            Ok(df) => df,
            Err(e) => return Err(format!("Failed to sort DataFrame: {}", e)),
        };
    }

    let mut df_info = match process_dataframe(lf, file_path) {
        Ok(df_info) => df_info,
        Err(e) => return Err(format!("Failed to process DataFrame: {}", e)),
    };

    // Update shape after filtering
    df_info.shape = shape;

    Ok(df_info)
}

#[command]
fn get_more_rows(file_path: &str, offset: usize, limit: usize, sorting: Option<Vec<Sorting>>, filtering: Option<Vec<Filtering>>) -> Result<Vec<Vec<String>>, String> {
    let mut lf = match open_parquet(file_path) {
        Ok(df) => df,
        Err(e) => return Err(format!("Failed to read Parquet file: {}", e)),
    };

    if let Some(filtering_info) = filtering {
        lf = match filter_columns(lf, filtering_info) {
            Ok(df) => df,
            Err(e) => return Err(format!("Failed to filter DataFrame: {}", e)),
        };
    }

    if let Some(sorting_info) = sorting {
        lf = match sort_columns(lf, sorting_info) {
            Ok(df) => df,
            Err(e) => return Err(format!("Failed to sort DataFrame: {}", e)),
        };
    }

    let df = collect_dataframe_safe(lf)?;
    let df_slice = df.slice(offset as i64, limit);
    let rows = dataframe_to_rows(&df_slice);
    Ok(rows)
}
#[command]
fn get_statistics(file_path: &str) -> Result<HashMap<String, HashMap<String, serde_json::Value>>, String> {
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

        // Check if column is numeric, date, or Enum
        if column.dtype().is_numeric()
            || column.dtype() == &polars::prelude::DataType::Date
            || column.dtype() == &polars::prelude::DataType::Datetime(polars::prelude::TimeUnit::Milliseconds, None)
        {
            // Null values count
            let null_values = column.null_count() as f64;
            column_stats.insert("null_values".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(null_values).unwrap()));

            // Min, Max, Mean, Median, 25th and 75th Percentile
            if let Ok(Some(min_value)) = column.min::<f64>() {
                column_stats.insert("min".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(min_value).unwrap()));
            }

            if let Ok(Some(max_value)) = column.max::<f64>() {
                column_stats.insert("max".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(max_value).unwrap()));
            }

            if let Some(mean) = column.mean() {
                column_stats.insert("mean".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(mean).unwrap()));
            }

            if let Some(median) = column.median() {
                column_stats.insert("median".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(median).unwrap()));
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
                        column_stats.insert("percentile_25".to_string(), serde_json::Value::String(p25_value.to_string()));
                    }

                    if let Ok(p75_value) = sorted_column.get(p75_idx) {
                        column_stats.insert("percentile_75".to_string(), serde_json::Value::String(p75_value.to_string()));
                    }
                }
            }
        } else if let polars::prelude::DataType::Enum(Some(rev_mapping), _) = column.dtype() {
            // Enum column

            // Unique values count (distinct count)
            let unique_values = match column.n_unique() {
                Ok(val) => serde_json::Value::Number(serde_json::Number::from(val as u64)),
                Err(e) => return Err(format!("Failed to compute unique values for column '{}': {}", column_name, e)),
            };
            column_stats.insert("unique_values".to_string(), unique_values);

            // Null values count
            let null_values = column.null_count() as f64;
            column_stats.insert("null_values".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(null_values).unwrap()));

            // Available Enum values
            let enum_values: Vec<String> = (0..rev_mapping.len())
                .filter_map(|ix| ix.try_into().ok().map(|ix_u32| rev_mapping.get(ix_u32)))
                .map(|s| s.to_string())
                .collect();
            column_stats.insert("available_values".to_string(), serde_json::Value::Array(enum_values.into_iter().map(serde_json::Value::String).collect()));
        } else {
            // Non-numeric column
            // Unique values count (distinct count)
            let unique_values = match column.n_unique() {
                Ok(val) => serde_json::Value::Number(serde_json::Number::from(val as u64)),
                Err(e) => return Err(format!("Failed to compute unique values for column '{}': {}", column_name, e)),
            };
            column_stats.insert("unique_values".to_string(), unique_values);

            // Null values count
            let null_values = column.null_count() as f64;
            column_stats.insert("null_values".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(null_values).unwrap()));
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
