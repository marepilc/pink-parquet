//! Query engines behind the v3 data-source layer.
//!
//! For now this only holds the DuckDB connection helpers used by the
//! phase-0 spike; the `DataSource` abstraction lands in phase 1.

pub mod duckdb_engine;

pub use duckdb_engine::duckdb_version;
