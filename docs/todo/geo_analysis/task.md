# Geo Analysis (Phase 4.6) Tasklist

## 1. Web-Analyzer Core Refactor
- [ ] Append `Option<tokio::sync::mpsc::Sender<ScanProgress>>` param into `web_analyzer::geo_analysis::analyze_geometry`.
- [ ] Send status updates mapping IP lookup structures.
- [ ] Transmit events representing `llms.txt` and crawler directive scraping statuses.

## 2. Cargo & WebQ Backend
- [ ] Hook `src-tauri/src/lib.rs` for proxy wrapper `scan_geo_analysis`.
- [ ] Emplace channel forwarding map loop -> `app_handle.emit`.

## 3. WebQ Frontend UI
- [ ] Overlay the `<ScanTerminal />` loading state directly inside `src/routes/assessment/geo-analysis/+page.svelte`.
- [ ] Integrate local `$state` mappings matching standard layout conventions.
