use rusqlite::Connection;
use std::path::Path;

/// Opens (creating if needed) the SQLite file at `db_path` and ensures the
/// `applications` schema exists. Safe to call every launch — all statements
/// are idempotent (`IF NOT EXISTS`).
pub fn init_connection(db_path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS applications (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            company     TEXT NOT NULL,
            role        TEXT NOT NULL,
            status      TEXT NOT NULL DEFAULT 'applied',
            job_url     TEXT,
            location    TEXT,
            salary      TEXT,
            applied_on  TEXT,
            notes       TEXT,
            created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
            updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
        );

        -- Keeps updated_at current on every edit. Relies on SQLite's default
        -- recursive_triggers=OFF, so the UPDATE below does not re-fire itself.
        CREATE TRIGGER IF NOT EXISTS applications_set_updated_at
        AFTER UPDATE ON applications
        FOR EACH ROW
        BEGIN
            UPDATE applications
            SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
            WHERE id = OLD.id;
        END;
        ",
    )?;

    // CREATE TABLE IF NOT EXISTS only applies the full schema to brand-new
    // databases — existing installs need these columns added explicitly.
    add_column_if_missing(&conn, "work_mode", "TEXT")?;
    add_column_if_missing(&conn, "contract_type", "TEXT")?;

    Ok(conn)
}

fn add_column_if_missing(conn: &Connection, column: &str, sql_type: &str) -> rusqlite::Result<()> {
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('applications') WHERE name = ?1",
        [column],
        |row| row.get(0),
    )?;

    if exists == 0 {
        conn.execute(
            &format!("ALTER TABLE applications ADD COLUMN {column} {sql_type}"),
            [],
        )?;
    }

    Ok(())
}
