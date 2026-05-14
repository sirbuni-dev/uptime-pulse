use crate::db::{self, HeartbeatEvent};
use reqwest::Client;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::{AppHandle, Emitter};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

pub const STATUS_DOWN: i64 = 0;
pub const STATUS_UP: i64 = 1;

pub struct Scheduler {
    pub handles: Mutex<std::collections::HashMap<i64, JoinHandle<()>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            handles: Mutex::new(std::collections::HashMap::new()),
        }
    }

    pub fn spawn_monitor(
        &self,
        monitor_id: i64,
        url: String,
        interval: u64,
        timeout: u64,
        db: Arc<Mutex<Connection>>,
        app_handle: AppHandle,
    ) {
        let handle = tokio::spawn(async move {
            let client = Client::builder()
                .timeout(Duration::from_secs(timeout))
                .danger_accept_invalid_certs(false)
                .use_rustls_tls()
                .build()
                .unwrap_or_default();

            loop {
                let start = Instant::now();
                let (status, ping, msg) = check_http(&client, &url).await;
                let ping_ms = start.elapsed().as_millis() as f64;
                let actual_ping = if status == STATUS_UP { Some(ping_ms) } else { ping };

                let beat = {
                    let conn = db.lock().unwrap();
                    db::insert_heartbeat(&conn, monitor_id, status, actual_ping, msg.as_deref())
                        .unwrap_or_else(|e| {
                            eprintln!("db insert error for monitor {monitor_id}: {e}");
                            db::HeartbeatDto {
                                id: 0,
                                monitor_id,
                                status,
                                ping: actual_ping,
                                msg: msg.clone(),
                                checked_at: chrono::Utc::now()
                                    .format("%Y-%m-%dT%H:%M:%SZ")
                                    .to_string(),
                            }
                        })
                };

                let event = HeartbeatEvent {
                    monitor_id: beat.monitor_id,
                    status:     beat.status,
                    ping:       beat.ping,
                    msg:        beat.msg.clone(),
                    checked_at: beat.checked_at.clone(),
                };

                if let Err(e) = app_handle.emit("monitor:heartbeat", &event) {
                    eprintln!("emit error for monitor {monitor_id}: {e}");
                }

                sleep(Duration::from_secs(interval)).await;
            }
        });

        self.handles.lock().unwrap().insert(monitor_id, handle);
    }

    pub fn stop_monitor(&self, monitor_id: i64) {
        if let Some(handle) = self.handles.lock().unwrap().remove(&monitor_id) {
            handle.abort();
        }
    }

    #[allow(dead_code)]
    pub fn stop_all(&self) {
        let mut handles = self.handles.lock().unwrap();
        for (_, handle) in handles.drain() {
            handle.abort();
        }
    }
}

async fn check_http(client: &Client, url: &str) -> (i64, Option<f64>, Option<String>) {
    match client.get(url).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                (STATUS_UP, None, None)
            } else {
                let code = resp.status().as_u16();
                (STATUS_DOWN, None, Some(format!("HTTP {code}")))
            }
        }
        Err(e) => {
            let msg = if e.is_timeout() {
                "Timeout".to_string()
            } else if e.is_connect() {
                "Connection refused".to_string()
            } else {
                e.to_string()
            };
            (STATUS_DOWN, None, Some(msg))
        }
    }
}
