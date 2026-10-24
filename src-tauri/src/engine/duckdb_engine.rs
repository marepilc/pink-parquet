use duckdb::Connection;

/// Opens an in-memory DuckDB database
pub fn open_in_memory() -> Result<Connection, String> {
    Connection::open_in_memory().map_err(|e| format!("Failed to open DuckDB: {}", e))
}

/// Returns the version of the bundled DuckDB library (e.g. "v1.4.1")
pub fn duckdb_version() -> Result<String, String> {
    let conn = open_in_memory()?;
    conn.query_row("SELECT version()", [], |row| row.get::<_, String>(0))
        .map_err(|e| format!("Failed to read DuckDB version: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("pinkparquet-{}-{}", std::process::id(), name))
    }

    #[test]
    fn reports_version() {
        let version = duckdb_version().unwrap();
        assert!(version.starts_with('v'), "unexpected version: {version}");
    }

    // The parquet and json extensions must be compiled in: a release build
    // cannot rely on downloading them at runtime.
    #[test]
    fn reads_parquet_without_network() {
        let conn = open_in_memory().unwrap();
        conn.execute_batch("SET autoinstall_known_extensions = false; SET autoload_known_extensions = false;")
            .unwrap();

        let path = temp_path("roundtrip.parquet");
        let path_str = path.to_string_lossy().replace('\'', "''");
        conn.execute_batch(&format!(
            "COPY (SELECT range AS i, 'row ' || range AS label FROM range(1000)) TO '{path_str}' (FORMAT parquet)"
        ))
        .unwrap();

        let (count, sum): (i64, i64) = conn
            .query_row(
                &format!("SELECT count(*), sum(i)::BIGINT FROM read_parquet('{path_str}')"),
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(count, 1000);
        assert_eq!(sum, 499_500);
    }

    #[test]
    fn reads_json_without_network() {
        let conn = open_in_memory().unwrap();
        conn.execute_batch("SET autoinstall_known_extensions = false; SET autoload_known_extensions = false;")
            .unwrap();

        let path = temp_path("rows.json");
        std::fs::write(&path, r#"[{"a": 1}, {"a": 2}, {"a": 3}]"#).unwrap();
        let path_str = path.to_string_lossy().replace('\'', "''");

        let total: i64 = conn
            .query_row(
                &format!("SELECT sum(a)::BIGINT FROM read_json('{path_str}')"),
                [],
                |row| row.get(0),
            )
            .unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(total, 6);
    }
}
