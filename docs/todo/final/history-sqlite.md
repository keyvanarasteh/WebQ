# Scan History And SQLite

## Status

Partially implemented. Double-checked against `src-tauri/src/db.rs`, `src-tauri/src/lib.rs`, `/history`, and `/history/[id]` on 2026-07-06.

## Implemented

- [x] SQLite initialization with WAL mode in `src-tauri/src/db.rs`.
- [x] `scans` table with target/module/status/error/favorite/duration/timestamps.
- [x] `scan_results` table with raw JSON blob and vulnerability counters.
- [x] Cascade relationship from scan results to scans.
- [x] `log_scan_to_db` wrapper used by most scan commands.
- [x] `get_scans_paginated(...)` with pagination, domain/module/status/date filters, and allowlisted sorting.
- [x] `get_scan_blob_details(scan_id)`.
- [x] `delete_scan(id)`.
- [x] `toggle_favorite(id)` and star button wiring in `/history`.
- [x] `bulk_delete_scans(ids)` and multi-select UI in `/history`.
- [x] `get_global_statistics()` for scan totals, favorites, average score, vulnerability totals, top domains, and module usage.
- [x] DB size/count stats and nuke-history command for Settings.
- [x] `get_unique_scanned_domains()` exists for local domain hydration/autocomplete-style reuse.
- [x] `/history` table route and `/history/[id]` structured summary plus raw JSON detail route.
- [x] Browser Blob JSON export exists in `/history/[id]`.
- [x] Main dashboard links to Scan History.
- [x] Domain Info and Domain DNS hydrate latest local results.

## Remaining Tasks

- [x] Accept the current browser Blob export for now instead of adding a backend/native-save command.
- [x] Replace `/history/[id]` raw JSON-only view with structured summary/object sections plus raw JSON.
- [ ] Add module-specific renderers in `/history/[id]` for the highest-value scan result shapes.
- [ ] Add a real Recent Scans widget on `src/routes/+page.svelte`, not just a link card.
- [ ] Show "Saved to DB" confirmation after scans complete, where it will not create notification noise.
- [x] Fill `security_score`, `medium_vulns`, and `low_vulns` from known result keys and nested severity data when available.
- [ ] Decide whether failed scans should write an empty `scan_results` row or only a `scans` row.
