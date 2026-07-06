# Scan History And SQLite

## Status

Partially implemented.

## Implemented

- [x] SQLite initialization with WAL mode in `src-tauri/src/db.rs`.
- [x] `scans` table with target/module/status/error/favorite/duration/timestamps.
- [x] `scan_results` table with raw JSON blob and vulnerability counters.
- [x] Cascade relationship from scan results to scans.
- [x] `log_scan_to_db` wrapper used by most scan commands.
- [x] `get_scans_paginated(limit, offset)`.
- [x] `get_scan_blob_details(scan_id)`.
- [x] `delete_scan(id)`.
- [x] DB size/count stats and nuke-history command for Settings.
- [x] `/history` table route and `/history/[id]` raw JSON detail route.
- [x] Main dashboard links to Scan History.
- [x] Domain Info and Domain DNS hydrate latest local results.

## Remaining Tasks

- [ ] Extend `get_scans_paginated` with `filter_domain`, `filter_module`, date range, status, and sort options.
- [ ] Add `get_global_statistics()` for recent average score, top domains, module usage, and vulnerability counts.
- [ ] Add `toggle_favorite(id)` and wire the star button in `/history`.
- [ ] Add `bulk_delete_scans(ids)` and multi-select UI.
- [ ] Add `export_scan_json(id)` as a backend/native-save command, or document the current browser Blob export as accepted.
- [ ] Replace `/history/[id]` raw JSON-only view with a structured module-aware report view.
- [ ] Add a real Recent Scans widget on `src/routes/+page.svelte`, not just a link card.
- [ ] Show "Saved to DB" confirmation after scans complete, where it will not create notification noise.
- [ ] Fill `security_score`, `medium_vulns`, and `low_vulns` when modules provide that data.
- [ ] Decide whether failed scans should write an empty `scan_results` row or only a `scans` row.

