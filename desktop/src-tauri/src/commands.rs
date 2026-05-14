use crate::db::{self, CreateMonitorPayload, HeartbeatDto, MonitorDto, UpdateMonitorPayload};
use crate::scheduler::Scheduler;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};

pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub scheduler: Arc<Scheduler>,
}

type Db<'a> = State<'a, AppState>;

#[tauri::command]
pub async fn list_monitors(state: Db<'_>) -> Result<Vec<MonitorDto>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::list_monitors(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_monitor(
    state: Db<'_>,
    app_handle: AppHandle,
    payload: CreateMonitorPayload,
) -> Result<MonitorDto, String> {
    let monitor = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        db::create_monitor(&conn, &payload).map_err(|e| e.to_string())?
    };
    state.scheduler.spawn_monitor(
        monitor.id,
        monitor.url.clone(),
        monitor.interval as u64,
        monitor.timeout as u64,
        Arc::clone(&state.db),
        app_handle,
    );
    Ok(monitor)
}

#[tauri::command]
pub async fn update_monitor(
    state: Db<'_>,
    app_handle: AppHandle,
    payload: UpdateMonitorPayload,
) -> Result<MonitorDto, String> {
    let monitor = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        db::update_monitor(&conn, &payload).map_err(|e| e.to_string())?
    };
    state.scheduler.stop_monitor(monitor.id);
    if monitor.active {
        state.scheduler.spawn_monitor(
            monitor.id,
            monitor.url.clone(),
            monitor.interval as u64,
            monitor.timeout as u64,
            Arc::clone(&state.db),
            app_handle,
        );
    }
    Ok(monitor)
}

#[tauri::command]
pub async fn delete_monitor(state: Db<'_>, id: i64) -> Result<(), String> {
    state.scheduler.stop_monitor(id);
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::delete_monitor(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_active(
    state: Db<'_>,
    app_handle: AppHandle,
    id: i64,
    active: bool,
) -> Result<MonitorDto, String> {
    let monitor = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        let m = db::get_monitor_by_id(&conn, id).map_err(|e| e.to_string())?;
        let payload = UpdateMonitorPayload {
            id: m.id,
            name: m.name.clone(),
            url: m.url.clone(),
            interval: m.interval,
            timeout: m.timeout,
            active,
        };
        db::update_monitor(&conn, &payload).map_err(|e| e.to_string())?
    };
    state.scheduler.stop_monitor(id);
    if active {
        state.scheduler.spawn_monitor(
            monitor.id,
            monitor.url.clone(),
            monitor.interval as u64,
            monitor.timeout as u64,
            Arc::clone(&state.db),
            app_handle,
        );
    }
    Ok(monitor)
}

#[tauri::command]
pub async fn get_heartbeats(
    state: Db<'_>,
    monitor_id: i64,
    limit: Option<i64>,
) -> Result<Vec<HeartbeatDto>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::get_heartbeats(&conn, monitor_id, limit.unwrap_or(90)).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_uptime_percentage(
    state: Db<'_>,
    monitor_id: i64,
    hours: Option<i64>,
) -> Result<f64, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::get_uptime_percentage(&conn, monitor_id, hours.unwrap_or(24)).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_all_monitors(
    state: Db<'_>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let monitors = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        db::list_monitors(&conn).map_err(|e| e.to_string())?
    };
    for m in monitors.into_iter().filter(|m| m.active) {
        state.scheduler.spawn_monitor(
            m.id,
            m.url.clone(),
            m.interval as u64,
            m.timeout as u64,
            Arc::clone(&state.db),
            app_handle.clone(),
        );
    }
    Ok(())
}

#[tauri::command]
pub async fn pause_monitor(state: Db<'_>, id: i64) -> Result<(), String> {
    state.scheduler.stop_monitor(id);
    Ok(())
}

#[tauri::command]
pub async fn resume_monitor(
    state: Db<'_>,
    app_handle: AppHandle,
    id: i64,
) -> Result<(), String> {
    let monitor = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        db::get_monitor_by_id(&conn, id).map_err(|e| e.to_string())?
    };
    state.scheduler.spawn_monitor(
        monitor.id,
        monitor.url,
        monitor.interval as u64,
        monitor.timeout as u64,
        Arc::clone(&state.db),
        app_handle,
    );
    Ok(())
}
