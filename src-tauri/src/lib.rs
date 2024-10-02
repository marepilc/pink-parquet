mod data_loader;

use tauri::command;
use data_loader::{open_parquet, process_dataframe, DataFrameInfo, dataframe_to_rows, collect_dataframe_safe};
use data_loader::dataframe_processor::{sort_columns, Sorting};

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
        let df_info = match process_dataframe(df) {
            Ok(df_info) => df_info,
            Err(e) => return Err(format!("Failed to process DataFrame: {}", e)),
        };
        return Ok(df_info);
    }

    let df_info = match process_dataframe(lf) {
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
        .invoke_handler(tauri::generate_handler![get_data, get_more_rows])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
