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

## Gaps

- [ ] `/react` dashboard uses mock KPIs, mock live events, and mock vector distribution.
- [ ] `/react/attackers` uses mock attacker profiles instead of `get_top_attackers`.
- [ ] `/react/signatures` only includes a small sample of the `REQUEST.md` signature matrix.
- [ ] `testPayload()` in `/react/honeypot` calls `test_payload_locally` with `{ payload }`, but the backend command requires `method`, `path`, `queryString`, `body`, and `headers`.
- [ ] Honeypot events are session-local and not persisted to SQLite.
- [ ] There is no full attacker/event history table with filters.
- [ ] React2Shell scanner/source-leak/RCE commands return final results but do not emit `scan-progress`.

## Tasks

- [ ] Fix `/react/honeypot` local payload tester payload shape.
- [ ] Add inputs for method/path/query/body/headers or map a single payload into a default POST body intentionally.
- [ ] Wire `/react` dashboard to live event state or backend summary commands.
- [ ] Wire `/react/attackers` to `get_top_attackers`.
- [ ] Replace `/react/signatures` sample data with the full 40+ signature matrix, preferably generated from a shared data file.
- [ ] Persist attack events and attacker snapshots to SQLite if historical analysis is required.
- [ ] Add typed frontend interfaces for `AttackEvent`, `AttackerProfile`, and honeypot config.
- [ ] Decide whether React2Shell scanner/source-leak/RCE should emit `scan-progress`, or document them as final-result-only commands.
