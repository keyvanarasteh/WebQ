# Nmap Zero Day (Phase 4.4) Tasklist

## 1. Web-Analyzer Core Refactor
- [ ] Refit `web_analyzer::nmap_zero_day::run_cve_scan` to take `progress_tx: Option<tokio::sync::mpsc::Sender<ScanProgress>>`.
- [ ] Add real-time hook for Nmap initialization step.
- [ ] Inject update steps during Nmap XML Parsing phase.
- [ ] Insert granular `tx.send` updates mapping queried versions against the local or remote NVD CVE database.

## 2. Cargo & WebQ Backend
- [ ] Confirm integration logic holds within `scan_nmap_zero_day` inside `src-tauri/src/lib.rs`.
- [ ] Propagate asynchronous channel payload straight to the global web socket UI via `app_handle.emit`.

## 3. WebQ Frontend UI
- [ ] Set up `<ScanTerminal>` within `src/routes/assessment/nmap-zero-day/+page.svelte`.
- [ ] Emplace standard Svelte `$state` and listener API payload tracking for the Nmap event cycle.
- [ ] Specifically verify that the Nmap ETA/Progress bar maps to the overarching progress percent variable gracefully.
