use anyhow::Result;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::Path;

const SCHEMA: &str = "
    CREATE TABLE IF NOT EXISTS monitors (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        name        TEXT    NOT NULL,
        url         TEXT    NOT NULL,
        type        TEXT    NOT NULL DEFAULT 'http',
        interval    INTEGER NOT NULL DEFAULT 60,
        timeout     INTEGER NOT NULL DEFAULT 10,
        active      INTEGER NOT NULL DEFAULT 1,
        created_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
        updated_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
    );

    CREATE TABLE IF NOT EXISTS heartbeats (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        monitor_id  INTEGER NOT NULL,
        status      INTEGER NOT NULL,
        ping        REAL,
        msg         TEXT,
        checked_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
        FOREIGN KEY (monitor_id) REFERENCES monitors(id) ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_heartbeats_monitor_checked
        ON heartbeats(monitor_id, checked_at DESC);

    PRAGMA foreign_keys = ON;
    PRAGMA journal_mode = WAL;
";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorDto {
    pub id: i64,
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub monitor_type: String,
    pub interval: i64,
    pub timeout: i64,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatDto {
    pub id: i64,
    pub monitor_id: i64,
    pub status: i64,
    pub ping: Option<f64>,
    pub msg: Option<String>,
    pub checked_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatEvent {
    pub monitor_id: i64,
    pub status: i64,
    pub ping: Option<f64>,
    pub msg: Option<String>,
    pub checked_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMonitorPayload {
    pub name: String,
    pub url: String,
    pub interval: Option<i64>,
    pub timeout: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMonitorPayload {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub interval: i64,
    pub timeout: i64,
    pub active: bool,
}

pub fn open(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(SCHEMA)?;
    Ok(conn)
}

pub fn list_monitors(conn: &Connection) -> Result<Vec<MonitorDto>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, url, type, interval, timeout, active, created_at, updated_at
         FROM monitors ORDER BY id ASC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(MonitorDto {
            id:           row.get(0)?,
            name:         row.get(1)?,
            url:          row.get(2)?,
            monitor_type: row.get(3)?,
            interval:     row.get(4)?,
            timeout:      row.get(5)?,
            active:       row.get::<_, i64>(6)? != 0,
            created_at:   row.get(7)?,
            updated_at:   row.get(8)?,
        })
    })?;
    rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
}

pub fn create_monitor(conn: &Connection, p: &CreateMonitorPayload) -> Result<MonitorDto> {
    conn.execute(
        "INSERT INTO monitors (name, url, interval, timeout) VALUES (?1, ?2, ?3, ?4)",
        params![p.name, p.url, p.interval.unwrap_or(60), p.timeout.unwrap_or(10)],
    )?;
    let id = conn.last_insert_rowid();
    get_monitor_by_id(conn, id)
}

pub fn update_monitor(conn: &Connection, p: &UpdateMonitorPayload) -> Result<MonitorDto> {
    conn.execute(
        "UPDATE monitors SET name=?1, url=?2, interval=?3, timeout=?4, active=?5,
         updated_at=strftime('%Y-%m-%dT%H:%M:%SZ','now') WHERE id=?6",
        params![p.name, p.url, p.interval, p.timeout, p.active as i64, p.id],
    )?;
    get_monitor_by_id(conn, p.id)
}

pub fn delete_monitor(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM monitors WHERE id=?1", params![id])?;
    Ok(())
}

pub fn get_monitor_by_id(conn: &Connection, id: i64) -> Result<MonitorDto> {
    conn.query_row(
        "SELECT id, name, url, type, interval, timeout, active, created_at, updated_at
         FROM monitors WHERE id=?1",
        params![id],
        |row| {
            Ok(MonitorDto {
                id:           row.get(0)?,
                name:         row.get(1)?,
                url:          row.get(2)?,
                monitor_type: row.get(3)?,
                interval:     row.get(4)?,
                timeout:      row.get(5)?,
                active:       row.get::<_, i64>(6)? != 0,
                created_at:   row.get(7)?,
                updated_at:   row.get(8)?,
            })
        },
    )
    .map_err(Into::into)
}

pub fn insert_heartbeat(
    conn: &Connection,
    monitor_id: i64,
    status: i64,
    ping: Option<f64>,
    msg: Option<&str>,
) -> Result<HeartbeatDto> {
    conn.execute(
        "INSERT INTO heartbeats (monitor_id, status, ping, msg) VALUES (?1, ?2, ?3, ?4)",
        params![monitor_id, status, ping, msg],
    )?;
    let id = conn.last_insert_rowid();

    // Keep only the last 90 heartbeats per monitor
    conn.execute(
        "DELETE FROM heartbeats WHERE monitor_id=?1 AND id NOT IN (
             SELECT id FROM heartbeats WHERE monitor_id=?1
             ORDER BY checked_at DESC LIMIT 90
         )",
        params![monitor_id],
    )?;

    conn.query_row(
        "SELECT id, monitor_id, status, ping, msg, checked_at FROM heartbeats WHERE id=?1",
        params![id],
        |row| {
            Ok(HeartbeatDto {
                id:         row.get(0)?,
                monitor_id: row.get(1)?,
                status:     row.get(2)?,
                ping:       row.get(3)?,
                msg:        row.get(4)?,
                checked_at: row.get(5)?,
            })
        },
    )
    .map_err(Into::into)
}

pub fn get_heartbeats(conn: &Connection, monitor_id: i64, limit: i64) -> Result<Vec<HeartbeatDto>> {
    let mut stmt = conn.prepare(
        "SELECT id, monitor_id, status, ping, msg, checked_at
         FROM heartbeats WHERE monitor_id=?1
         ORDER BY checked_at DESC LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![monitor_id, limit], |row| {
        Ok(HeartbeatDto {
            id:         row.get(0)?,
            monitor_id: row.get(1)?,
            status:     row.get(2)?,
            ping:       row.get(3)?,
            msg:        row.get(4)?,
            checked_at: row.get(5)?,
        })
    })?;
    rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
}

pub fn get_uptime_percentage(conn: &Connection, monitor_id: i64, hours: i64) -> Result<f64> {
    let row: (i64, i64) = conn.query_row(
        "SELECT
             COUNT(*) AS total,
             SUM(CASE WHEN status = 1 THEN 1 ELSE 0 END) AS up_count
         FROM heartbeats
         WHERE monitor_id = ?1
           AND checked_at >= strftime('%Y-%m-%dT%H:%M:%SZ', 'now', ?2)",
        params![monitor_id, format!("-{} hours", hours)],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    if row.0 == 0 {
        return Ok(0.0);
    }
    Ok(row.1 as f64 / row.0 as f64 * 100.0)
}
