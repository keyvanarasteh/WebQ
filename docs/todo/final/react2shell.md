# React2Shell Honeypot And UI

## Status

Partially implemented. Double-checked against `src-tauri/src/honeypot.rs`, `src-tauri/src/lib.rs`, `/react`, `/react/honeypot`, `/react/attackers`, and `/react/signatures` on 2026-07-06.

## Implemented

- [x] `src-tauri/src/honeypot.rs` wraps `web_analyzer::react_honeypot::HoneypotEngine`.
- [x] Tauri commands exist for starting/stopping the honeypot, checking status, getting top attackers, local payload tests, and config updates.
- [x] The Axum honeypot emits `honeypot-attack-detected` events.
- [x] `/react/honeypot` can start and stop the daemon and listens for live attack detections.
- [x] `/react/honeypot` calls `get_top_attackers` for the local page state.
- [x] `/react/honeypot` includes a config panel and architecture documentation modal.
- [x] React2Shell scanner/source-leak/RCE Tauri commands are logged through SQLite history.
- [x] `/react` dashboard listens for `honeypot-attack-detected` events and derives KPIs/vector distribution from live state.
- [x] `/react/attackers` loads attacker profiles from `get_top_attackers`.
- [x] `/react/signatures` uses a shared full 40+ signature matrix.
- [x] `/react/honeypot` local payload tester now sends the backend-required request shape.
- [x] Frontend interfaces are typed for live attack events and attacker profiles.

## Gaps

- [ ] Honeypot events are session-local and not persisted to SQLite.
- [ ] There is no full attacker/event history table with filters.
- [x] React2Shell scanner/source-leak/RCE commands are documented as final-result-only actions for now.

## Tasks

- [x] Fix `/react/honeypot` local payload tester payload shape.
- [x] Map the single payload tester input into an intentional default POST request body.
- [x] Wire `/react` dashboard to live event state.
- [x] Wire `/react/attackers` to `get_top_attackers`.
- [x] Replace `/react/signatures` sample data with the full 40+ signature matrix in a shared data file.
- [ ] Persist attack events and attacker snapshots to SQLite if historical analysis is required.
- [x] Add typed frontend interfaces for `AttackEvent` and `AttackerProfile`.
- [x] Document React2Shell scanner/source-leak/RCE as final-result-only commands.
