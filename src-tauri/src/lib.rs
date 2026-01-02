use parquet::file::reader::FileReader;
use polars::prelude::*;
use polars_sql::SQLContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Emitter, Manager, Window};

mod data_loader;
use data_loader::{apply_sorts, calculate_statistics, dataframe_to_rows, open_parquet, Sorting};

#[derive(Default)]
pub struct AppState {
    cache: Mutex<Option<CacheEntry>>,
}

struct CacheEntry {
    file_path: Option<String>,
    sorting: Option<Vec<Sorting>>,
    query: Option<String>,
    df: DataFrame,
}

#[derive(Serialize)]
struct ColumnInfo {
    name: String,
    dtype: String,
}

#[derive(Serialize)]
struct MetadataInfo {
    name: String,
    created: Option<String>,
    modified: Option<String>,
    size: u64,
    row_groups: usize,
    compression: String,
    total_nulls: usize,
}

#[derive(Serialize)]
struct DataFrameInfo {
    shape: (usize, usize),
    columns: Vec<ColumnInfo>,
    rows: Vec<Vec<String>>,
    metadata: Option<MetadataInfo>,
}

fn table_name_from_path(path: &str) -> String {
    let stem = std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("table");
    let mut s = String::with_capacity(stem.len() + 1);

    for (i, ch) in stem.chars().enumerate() {
        if i == 0 && ch.is_ascii_digit() {
            s.push('_');
        }
        if ch.is_ascii_alphanumeric() || ch == '_' {
            s.push(ch);
        } else {
            s.push('_');
        }
    }

    if s.is_empty() {
        s.push_str("table");
    }
    s
}

// Helper function to extract column information from a DataFrame
fn extract_columns(df: &DataFrame) -> Vec<ColumnInfo> {
    df.get_columns()
        .iter()
        .map(|col| ColumnInfo {
            name: col.name().to_string(),
            dtype: format!("{:?}", col.dtype()),
        })
        .collect()
}

// Helper function to execute SQL query with proper error handling
fn execute_sql_query(
    ctx: &mut SQLContext,
    query: &str,
) -> Result<polars::prelude::LazyFrame, String> {
    ctx.execute(query)
        .map_err(|e| format!("SQL execution error: {}", e))
}

// Get initial data with metadata (returns first 100 rows)
#[tauri::command]
fn get_data(
    state: tauri::State<AppState>,
    file_path: String,
    sorting: Option<Vec<Sorting>>,
) -> Result<DataFrameInfo, String> {
    // Check cache
    {
        let cache = state.cache.lock().unwrap();
        if let Some(entry) = cache.as_ref() {
            if entry.file_path.as_deref() == Some(&file_path)
                && entry.sorting == sorting
                && entry.query.is_none()
            {
                let df = &entry.df;
                let shape = df.shape();
                let columns = extract_columns(df);

                let n = shape.0.min(100);
                let df_head = df.head(Some(n));
                let rows = dataframe_to_rows(&df_head)?;
                let mut metadata = extract_metadata(&file_path)?;
                metadata.total_nulls = calculate_total_nulls(df);

                return Ok(DataFrameInfo {
                    shape,
                    columns,
                    rows,
                    metadata: Some(metadata),
                });
            }
        }
    }

    // Open Parquet file
    let mut lf = open_parquet(&file_path)?;

    // Apply sorts if provided
    if let Some(sorts) = sorting.clone() {
        lf = apply_sorts(lf, sorts)?;
    }

    // Collect DataFrame
    let df = lf
        .collect()
        .map_err(|e| format!("Failed to collect DataFrame: {}", e))?;

    // Update cache
    {
        let mut cache = state.cache.lock().unwrap();
        *cache = Some(CacheEntry {
            file_path: Some(file_path.clone()),
            sorting,
            query: None,
            df: df.clone(),
        });
    }

    let shape = df.shape();

    // Get column information
    let columns = extract_columns(&df);

    // Get first 100 rows
    let n = shape.0.min(100);
    let df_head = df.head(Some(n));
    let rows = dataframe_to_rows(&df_head)?;

    // Extract metadata
    let mut metadata = extract_metadata(&file_path)?;
    metadata.total_nulls = calculate_total_nulls(&df);

    Ok(DataFrameInfo {
        shape,
        columns,
        rows,
        metadata: Some(metadata),
    })
}

// Get more rows with pagination (offset and limit)
#[tauri::command]
fn get_more_rows(
    state: tauri::State<AppState>,
    file_path: String,
    offset: usize,
    limit: usize,
    sorting: Option<Vec<Sorting>>,
) -> Result<Vec<Vec<String>>, String> {
    // Check cache
    {
        let cache = state.cache.lock().unwrap();
        if let Some(entry) = cache.as_ref() {
            if entry.file_path.as_deref() == Some(&file_path)
                && entry.sorting == sorting
                && entry.query.is_none()
            {
                let df = &entry.df;
                let df_slice = df.slice(offset as i64, limit);
                return dataframe_to_rows(&df_slice);
            }
        }
    }

    // Open Parquet file
    let mut lf = open_parquet(&file_path)?;

    // Apply sorts if provided
    if let Some(sorts) = sorting {
        lf = apply_sorts(lf, sorts)?;
    }

    // Apply pagination
    let lf = lf.slice(offset as i64, limit as u32);

    // Collect DataFrame
    let df = lf
        .collect()
        .map_err(|e| format!("Failed to collect DataFrame: {}", e))?;

    // Convert to rows
    dataframe_to_rows(&df)
}

// Execute SQL and return preview (paginated first page) replacing current table
#[tauri::command]
fn execute_sql(
    state: tauri::State<AppState>,
    active_file_path: String,
    all_files: Vec<String>,
    table_names: Option<HashMap<String, String>>,
    query: String,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Result<DataFrameInfo, String> {
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(100);

    // Check cache
    {
        let cache = state.cache.lock().unwrap();
        if let Some(entry) = cache.as_ref() {
            if entry.query.as_ref() == Some(&query) {
                let df = &entry.df;
                let shape = df.shape();
                let columns = extract_columns(df);

                let df_page = df.slice(offset as i64, limit);
                let rows = dataframe_to_rows(&df_page)?;
                let mut metadata = extract_metadata(&active_file_path)?;
                metadata.total_nulls = calculate_total_nulls(df);

                return Ok(DataFrameInfo {
                    shape,
                    columns,
                    rows,
                    metadata: Some(metadata),
                });
            }
        }
    }

    let mut ctx = SQLContext::new();

    // Register all open files as tables
    for path in &all_files {
        let lf = open_parquet(path)?;
        let table_name = if let Some(ref names) = table_names {
            names
                .get(path)
                .cloned()
                .unwrap_or_else(|| table_name_from_path(path))
        } else {
            table_name_from_path(path)
        };
        ctx.register(&table_name, lf);
    }

    // Build main query lazyframe
    let qlf = execute_sql_query(&mut ctx, &query)?;

    // Collect full DataFrame for the query result to allow efficient pagination/sorting
    let df = qlf
        .collect()
        .map_err(|e| format!("Failed to collect SQL result: {}", e))?;

    // Update cache
    {
        let mut cache = state.cache.lock().unwrap();
        *cache = Some(CacheEntry {
            file_path: None,
            sorting: None,
            query: Some(query.clone()),
            df: df.clone(),
        });
    }

    let shape = df.shape();

    // Get column information
    let columns = extract_columns(&df);

    let df_page = df.slice(offset as i64, limit);
    let rows = dataframe_to_rows(&df_page)?;

    // Keep file metadata so footer still shows file info
    let mut metadata = extract_metadata(&active_file_path)?;
    metadata.total_nulls = calculate_total_nulls(&df);

    Ok(DataFrameInfo {
        shape,
        columns,
        rows,
        metadata: Some(metadata),
    })
}

// Fetch more rows for the same SQL query (infinite scroll)
#[tauri::command]
fn get_more_sql_rows(
    state: tauri::State<AppState>,
    all_files: Vec<String>,
    table_names: Option<HashMap<String, String>>,
    query: String,
    offset: usize,
    limit: usize,
) -> Result<Vec<Vec<String>>, String> {
    // Check cache
    {
        let cache = state.cache.lock().unwrap();
        if let Some(entry) = cache.as_ref() {
            if entry.query.as_ref() == Some(&query) {
                let df = &entry.df;
                let df_slice = df.slice(offset as i64, limit);
                return dataframe_to_rows(&df_slice);
            }
        }
    }

    let mut ctx = SQLContext::new();

    // Register all open files as tables
    for path in &all_files {
        let lf = open_parquet(path)?;
        let table_name = if let Some(ref names) = table_names {
            names
                .get(path)
                .cloned()
                .unwrap_or_else(|| table_name_from_path(path))
        } else {
            table_name_from_path(path)
        };
        ctx.register(&table_name, lf);
    }

    let qlf = execute_sql_query(&mut ctx, &query)?;

    let page_df = qlf
        .slice(offset as i64, limit as u32)
        .collect()
        .map_err(|e| format!("Failed to collect SQL page: {}", e))?;

    dataframe_to_rows(&page_df)
}

// Get statistics for all columns
#[tauri::command]
fn get_statistics(
    file_path: String,
) -> Result<HashMap<String, HashMap<String, serde_json::Value>>, String> {
    // Open Parquet file
    let lf = open_parquet(&file_path)?;

    // Collect DataFrame
    let df = lf
        .collect()
        .map_err(|e| format!("Failed to collect DataFrame: {}", e))?;

    // Calculate statistics
    calculate_statistics(&df)
}

#[tauri::command]
fn get_query_statistics(
    state: tauri::State<AppState>,
    query: String,
) -> Result<HashMap<String, HashMap<String, serde_json::Value>>, String> {
    // Get the cached query result
    let cache = state.cache.lock().unwrap();
    if let Some(entry) = cache.as_ref() {
        if entry.query.as_ref() == Some(&query) {
            let df = &entry.df;
            return calculate_statistics(df);
        }
    }
    Err("Query result not found in cache".to_string())
}

// Extract file metadata
fn extract_metadata(file_path: &str) -> Result<MetadataInfo, String> {
    let file_metadata =
        fs::metadata(file_path).map_err(|e| format!("Failed to read file metadata: {}", e))?;

    let name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    let size = file_metadata.len();

    let created = file_metadata.created().ok().and_then(|t| {
        chrono::DateTime::<chrono::Local>::from(t)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .into()
    });

    let modified = file_metadata.modified().ok().and_then(|t| {
        chrono::DateTime::<chrono::Local>::from(t)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .into()
    });

    // Open the Parquet file to get row groups and compression info
    let file = fs::File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;

    let reader = parquet::file::serialized_reader::SerializedFileReader::new(file)
        .map_err(|e| format!("Failed to create Parquet reader: {}", e))?;

    let metadata = reader.metadata();
    let row_groups = metadata.num_row_groups();

    let compression = if row_groups > 0 {
        let rg_metadata = metadata.row_group(0);
        if rg_metadata.num_columns() > 0 {
            format!("{:?}", rg_metadata.column(0).compression())
        } else {
            "UNCOMPRESSED".to_string()
        }
    } else {
        "UNCOMPRESSED".to_string()
    };

    Ok(MetadataInfo {
        name,
        created,
        modified,
        size,
        row_groups,
        compression,
        total_nulls: 0, // Will be updated when we have DataFrame
    })
}

// Calculate total null count from DataFrame
fn calculate_total_nulls(df: &DataFrame) -> usize {
    df.get_columns()
        .iter()
        .map(|series| series.null_count())
        .sum()
}

// Save DataFrame to Parquet file
#[tauri::command]
fn save_parquet(state: tauri::State<AppState>, file_path: String) -> Result<(), String> {
    // Get the cached DataFrame
    let cache = state.cache.lock().unwrap();
    let df = cache
        .as_ref()
        .map(|entry| &entry.df)
        .ok_or_else(|| "No data to save".to_string())?;

    // Create parent directories if they don't exist
    if let Some(parent) = std::path::Path::new(&file_path).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Open file for writing
    let file = fs::File::create(&file_path).map_err(|e| format!("Failed to create file: {}", e))?;

    // Write DataFrame to Parquet
    polars::prelude::ParquetWriter::new(file)
        .finish(&mut df.clone())
        .map_err(|e| format!("Failed to write Parquet file: {}", e))?;

    Ok(())
}

// Legacy command for backward compatibility
#[tauri::command]
fn read_parquet(state: tauri::State<AppState>, file_path: String) -> Result<DataFrameInfo, String> {
    get_data(state, file_path, None)
}

// Settings structure
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Settings {
    theme: String,
    #[serde(default = "default_font_family", rename = "fontFamily")]
    font_family: String,
    #[serde(default = "default_font_size", rename = "fontSize")]
    font_size: u32,
}

fn default_font_family() -> String {
    "Iosevka".to_string()
}

fn default_font_size() -> u32 {
    14
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: "dark".to_string(),
            font_family: default_font_family(),
            font_size: default_font_size(),
        }
    }
}

// Get settings file path in app data directory
fn get_settings_path(app_handle: tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Create directory if it doesn't exist
    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    Ok(app_data_dir.join("settings.json"))
}

// Load settings from file
#[tauri::command]
fn load_settings(app_handle: tauri::AppHandle) -> Result<Settings, String> {
    let settings_path = get_settings_path(app_handle)?;

    if !settings_path.exists() {
        // Return default settings if file doesn't exist
        return Ok(Settings::default());
    }

    let contents = fs::read_to_string(&settings_path)
        .map_err(|e| format!("Failed to read settings file: {}", e))?;

    let settings: Settings =
        serde_json::from_str(&contents).map_err(|e| format!("Failed to parse settings: {}", e))?;

    Ok(settings)
}

// Save settings to file
#[tauri::command]
fn save_settings(app_handle: tauri::AppHandle, settings: Settings) -> Result<(), String> {
    let settings_path = get_settings_path(app_handle)?;

    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(&settings_path, json).map_err(|e| format!("Failed to write settings file: {}", e))?;

    Ok(())
}

#[derive(Serialize)]
struct HistogramData {
    bins: Vec<f64>,
    counts: Vec<usize>,
    min: f64,
    max: f64,
}

#[tauri::command]
fn get_column_histogram(
    file_path: String,
    column_name: String,
    num_bins: Option<usize>,
) -> Result<HistogramData, String> {
    // Open the file fresh to ensure we're working with the correct data
    let lf = open_parquet(&file_path)?;
    let df = lf
        .collect()
        .map_err(|e| format!("Failed to collect DataFrame: {}", e))?;

    calculate_histogram_from_dataframe(&df, &column_name, num_bins)
}

#[tauri::command]
fn get_query_column_histogram(
    state: tauri::State<AppState>,
    query: String,
    column_name: String,
    num_bins: Option<usize>,
) -> Result<HistogramData, String> {
    // Get the cached query result
    let cache = state.cache.lock().unwrap();
    if let Some(entry) = cache.as_ref() {
        if entry.query.as_ref() == Some(&query) {
            let df = &entry.df;
            return calculate_histogram_from_dataframe(df, &column_name, num_bins);
        }
    }
    Err("Query result not found in cache".to_string())
}

fn calculate_histogram_from_dataframe(
    df: &DataFrame,
    column_name: &str,
    num_bins: Option<usize>,
) -> Result<HistogramData, String> {
    // Get the column
    let series = df
        .column(&column_name)
        .map_err(|e| format!("Column not found: {}", e))?;

    // Convert to f64
    let ca = series
        .cast(&DataType::Float64)
        .map_err(|e| format!("Failed to convert to numeric: {}", e))?;
    let f64_series = ca
        .f64()
        .map_err(|e| format!("Failed to get f64 series: {}", e))?;

    // Get min and max
    let min = f64_series
        .min()
        .ok_or_else(|| "No min value in column".to_string())?;
    let max = f64_series
        .max()
        .ok_or_else(|| "No max value in column".to_string())?;

    // Calculate histogram
    let bins_count = num_bins.unwrap_or(20);

    // Handle edge case where all values are the same (min == max)
    if (max - min).abs() < f64::EPSILON {
        // All values are the same, create a single bin
        let bins = vec![min, min];
        let mut counts = vec![0usize; 1];

        // Count all non-null values
        for opt_val in f64_series.into_iter() {
            if opt_val.is_some() {
                counts[0] += 1;
            }
        }

        return Ok(HistogramData {
            bins,
            counts,
            min,
            max,
        });
    }

    let bin_width = (max - min) / bins_count as f64;

    let mut bins = Vec::new();
    let mut counts = vec![0usize; bins_count];

    // Create bin edges
    for i in 0..=bins_count {
        bins.push(min + i as f64 * bin_width);
    }

    // Count values in each bin
    for opt_val in f64_series.into_iter() {
        if let Some(val) = opt_val {
            if val >= min && val <= max {
                let bin_idx = ((val - min) / bin_width).floor() as usize;
                let bin_idx = bin_idx.min(bins_count - 1); // Handle edge case where val == max
                counts[bin_idx] += 1;
            }
        }
    }

    Ok(HistogramData {
        bins,
        counts,
        min,
        max,
    })
}

// Window control commands for custom title bar
#[tauri::command]
fn copy_full_table(state: tauri::State<AppState>) -> Result<String, String> {
    let cache = state.cache.lock().unwrap();
    if let Some(entry) = cache.as_ref() {
        let df = &entry.df;

        // Extract column names as header row
        let columns = extract_columns(df);
        let header = columns
            .iter()
            .map(|col| col.name.clone())
            .collect::<Vec<_>>()
            .join("\t");

        // Convert entire dataframe to TSV format
        let rows = dataframe_to_rows(df)?;
        let data_rows = rows
            .iter()
            .map(|row| row.join("\t"))
            .collect::<Vec<_>>()
            .join("\n");

        // Combine header and data
        let table_data = format!("{}\n{}", header, data_rows);

        Ok(table_data)
    } else {
        Err("No data loaded".to_string())
    }
}

#[tauri::command]
fn minimize_window(window: Window) {
    window.minimize().unwrap();
}

#[tauri::command]
fn maximize_window(window: Window) {
    window.maximize().unwrap();
}

#[tauri::command]
fn unmaximize_window(window: Window) {
    window.unmaximize().unwrap();
}

#[tauri::command]
fn close_window(window: Window) {
    window.close().unwrap();
}

#[tauri::command]
fn is_maximized(window: Window) -> bool {
    window.is_maximized().unwrap_or(false)
}

#[tauri::command]
fn read_text_file(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
            if args.len() > 1 {
                let _ = app.emit("open-file", args[1].clone());
            }
        }))
        .setup(|app: &mut tauri::App| {
            // Disable window decorations for custom title bar
            if let Some(_window) = app.get_webview_window("main") {
                #[cfg(target_os = "windows")]
                {
                    let _ = _window.set_decorations(false);
                }

                // On macOS, keep native decorations (traffic light buttons) visible
                // The titleBarStyle: "Overlay" in tauri.conf.json allows content under the title bar
            }

            // Handle file association on Windows
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let file_path = args[1].clone();
                if file_path.ends_with(".parquet") {
                    let app_handle = app.handle().clone();
                    // Give the frontend some time to load before emitting the event
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        let _ = app_handle.emit("open-file", file_path);
                    });
                }
            }
            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            read_parquet,
            get_data,
            get_more_rows,
            execute_sql,
            get_more_sql_rows,
            get_statistics,
            get_query_statistics,
            get_column_histogram,
            get_query_column_histogram,
            save_parquet,
            copy_full_table,
            load_settings,
            save_settings,
            minimize_window,
            maximize_window,
            unmaximize_window,
            close_window,
            is_maximized,
            read_text_file
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, _event| {
            // Handle file opening on macOS/iOS via "Open With" or double-click
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            if let tauri::RunEvent::Opened { urls } = _event {
                for url in urls {
                    if let Ok(path) = url.to_file_path() {
                        if path.to_string_lossy().ends_with(".parquet") {
                            let path_str = path.to_string_lossy().to_string();
                            let app_clone = _app.clone();
                            std::thread::spawn(move || {
                                std::thread::sleep(std::time::Duration::from_millis(1000));
                                let _ = app_clone.emit("open-file", path_str);
                            });
                        }
                    }
                }
            }
        });
}
