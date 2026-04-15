# Security Posture (Phase 4.1) Tasklist

## 1. Web-Analyzer Core Refactor
- [ ] Update `web_analyzer::security_analysis::analyze_security` parameters to accept `Option<tokio::sync::mpsc::Sender<ScanProgress>>`.
- [ ] Inject `tx.send` progress updates for WAF Identification phase.
- [ ] Inject progress updates for CORS Policy evaluation.
- [ ] Inject progress updates for Security Headers detection.
- [ ] Inject progress updates for HTTP Cookie inspection phase.

## 2. Cargo & WebQ Backend
- [ ] Ensure `web-analyzer` internal tests pass locally (`cargo check`).
- [ ] Modify `src-tauri/src/lib.rs` specific hook (`scan_security_analysis`) to spawn a listener looping `app_handle.emit("scan-progress", payload)`.

## 3. WebQ Frontend UI
- [ ] Navigate to `src/routes/assessment/security-posture/+page.svelte`.
- [ ] Import `listen` from `@tauri-apps/api/event` and `ScanTerminal` from UI primitives.
- [ ] Wire the `$state` bindings to dynamically track logs array and percentage score.
- [ ] Update responsive `<ScanTerminal />` to gracefully overlay or inline during active execution.
- [ ] Run full E2E assessment on a target to test seamless log UI visualization.
