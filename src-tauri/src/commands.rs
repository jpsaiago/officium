use rusqlite::{params, Connection, Row};
use std::sync::Mutex;
use tauri::State;

use crate::models::{Application, ApplicationStatus, NewApplication, UpdateApplication};

/// Tauri-managed state: one connection behind a mutex, since rusqlite's
/// `Connection` is not `Sync` and this app's write volume is far too low to
/// need a pool.
pub struct DbState(pub Mutex<Connection>);

fn row_to_application(row: &Row) -> rusqlite::Result<Application> {
    Ok(Application {
        id: row.get("id")?,
        company: row.get("company")?,
        role: row.get("role")?,
        status: row.get("status")?,
        job_url: row.get("job_url")?,
        location: row.get("location")?,
        work_mode: row.get("work_mode")?,
        contract_type: row.get("contract_type")?,
        salary: row.get("salary")?,
        applied_on: row.get("applied_on")?,
        notes: row.get("notes")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

#[tauri::command]
pub fn list_applications(state: State<DbState>) -> Result<Vec<Application>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT * FROM applications ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], row_to_application)
        .map_err(|e| e.to_string())?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_application(state: State<DbState>, id: i64) -> Result<Application, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT * FROM applications WHERE id = ?1",
        params![id],
        row_to_application,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_application(
    state: State<DbState>,
    payload: NewApplication,
) -> Result<Application, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let status = payload.status.unwrap_or(ApplicationStatus::Applied);

    conn.execute(
        "INSERT INTO applications (company, role, status, job_url, location, work_mode, contract_type, salary, applied_on, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            payload.company,
            payload.role,
            status,
            payload.job_url,
            payload.location,
            payload.work_mode,
            payload.contract_type,
            payload.salary,
            payload.applied_on,
            payload.notes,
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    conn.query_row(
        "SELECT * FROM applications WHERE id = ?1",
        params![id],
        row_to_application,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_application(
    state: State<DbState>,
    id: i64,
    payload: UpdateApplication,
) -> Result<Application, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let rows_affected = conn
        .execute(
            "UPDATE applications SET
                company       = COALESCE(?1, company),
                role          = COALESCE(?2, role),
                status        = COALESCE(?3, status),
                job_url       = COALESCE(?4, job_url),
                location      = COALESCE(?5, location),
                work_mode     = COALESCE(?6, work_mode),
                contract_type = COALESCE(?7, contract_type),
                salary        = COALESCE(?8, salary),
                applied_on    = COALESCE(?9, applied_on),
                notes         = COALESCE(?10, notes)
             WHERE id = ?11",
            params![
                payload.company,
                payload.role,
                payload.status,
                payload.job_url,
                payload.location,
                payload.work_mode,
                payload.contract_type,
                payload.salary,
                payload.applied_on,
                payload.notes,
                id,
            ],
        )
        .map_err(|e| e.to_string())?;

    if rows_affected == 0 {
        return Err(format!("no application found with id {id}"));
    }

    conn.query_row(
        "SELECT * FROM applications WHERE id = ?1",
        params![id],
        row_to_application,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_application(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let rows_affected = conn
        .execute("DELETE FROM applications WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    if rows_affected == 0 {
        return Err(format!("no application found with id {id}"));
    }
    Ok(())
}
