# Changelog

All notable changes to this project are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [desktop-v1.0.0] — 2026-05-15

### Added — Desktop App (Tauri v2 + Vue 3)

First release of the **Uptime Pulse Desktop** application, a native cross-platform
monitor client built with Tauri v2 and Vue 3 / TypeScript. Runs on macOS, Windows,
and Linux without requiring a server or Docker.

#### Core
- Tauri v2 shell with Vite + Vue 3 (TypeScript, Pinia, Vue Router) frontend
- SQLite persistence via `rusqlite` — database auto-created at the OS app-data path
- Background scheduler (Tokio) that pings each monitor on its configured interval
- IPC bridge: Rust commands exposed to the frontend (`list_monitors`, `create_monitor`,
  `update_monitor`, `delete_monitor`, `toggle_active`, `get_heartbeats`,
  `get_uptime_percentage`, `start_all_monitors`)
- Real-time heartbeat events pushed from Rust to Vue via Tauri `emit`

#### UI — Dashboard
- Resizable sidebar (drag handle, clamped 180–480 px) listing all monitors with
  live status dot, mini heartbeat bar (30 slots), and search/filter
- Detail panel with monitor header, action buttons (Pause/Resume, Edit, Clone, Delete),
  heartbeat bar, stats row (current ping, avg ping, 24 h / 30 d / 1 y uptime), Catmull-Rom
  spline ping chart (togglable), and paginated recent-checks table
- `ConfirmDialog` modal for delete confirmation (replaces unreliable native `confirm()`)
- `MonitorFormModal` for add / edit / clone workflows

#### UI — Theme & Appearance
- Light / dark theme toggle with FOUC-prevention script; all colours use CSS custom
  properties (`--bg`, `--text`, `--border`, …) defined on `:root` / `html.dark`
- Uptime Pulse brand icons across all platforms (generated via `cargo tauri icon`):
  PNG (16 → 512), ICNS (macOS), ICO (Windows), Windows Appx, iOS, Android
- Sidebar header logo and browser favicon set to the Uptime Pulse SVG

#### UI — Status System
- Status constants: `DOWN = 0`, `UP = 1`, `PENDING = 2`, `PAUSED = 3`
- `StatusBadge` component (green / red / amber / grey) used in detail header and
  recent-checks table
- Paused monitors display a grey "Paused" badge instead of the last recorded status

#### UI — Notifications
- Toast notification system (Pinia store + `ToastContainer` teleported to `<body>`)
- Types: `down` (red), `up` (green), `info` (blue), `error` (red)
- DOWN toast on every failed heartbeat; UP toast only on first recovery
- Info toast on successful monitor deletion; error toast if deletion fails
- Auto-dismiss after 10 s with slide-in / slide-out transition

#### UI — Heartbeat Bar & Tooltip
- Canvas-rendered pill bar (up to 40 visible beats, scales with container width)
- Hover tooltip (`position: fixed`, viewport coords) showing status label, local
  timestamp, message, and ping; CSS `::before` upward-triangle arrow

#### Timestamps
- All UTC ISO-8601 strings from the backend are converted to the system local timezone
  and formatted as human-readable 24-hour strings (e.g. `May 15, 2026, 07:43:24`)
  via `Intl.DateTimeFormat` / `toLocaleString({ hour12: false })`

#### CI — Release Workflow
- `.github/workflows/release-desktop.yml`: `workflow_dispatch` with `version` input
- Matrix builds: Ubuntu 22.04 (`.AppImage` / `.deb`), macOS universal
  (`--target universal-apple-darwin`, `.dmg`), Windows (`.msi` / `.exe`)
- Creates a draft GitHub release tagged `desktop-vX.Y.Z`
- Cargo build cache keyed on `Cargo.lock`

---

## [Unreleased] — Uptime Pulse Rebrand

### Changed
- Project renamed from **Uptime Kuma** to **Uptime Pulse** across all source files,
  Docker images, configuration keys, and documentation
- Bundle identifier updated to `com.ellynet.uptimepulse.desktop`

---

*Uptime Pulse Desktop is built on top of the Uptime Kuma open-source project.*
*See the original project at https://github.com/louislam/uptime-kuma*
