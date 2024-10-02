pub mod parquet_loader;
pub mod dataframe_processor;

// Re-export the key functions and types
pub use parquet_loader::{open_parquet, collect_dataframe};
pub use dataframe_processor::{process_dataframe, dataframe_to_rows, collect_dataframe_safe};
pub use dataframe_processor::DataFrameInfo;
