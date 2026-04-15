# API Security Scanner (Phase 4.5) Tasklist

## 1. Web-Analyzer Core Refactor
- [ ] Update `web_analyzer::api_security_scanner::fuzz_endpoints` arg sequence to pass `progress_tx: Option<tokio::sync::mpsc::Sender<ScanProgress>>`.
- [ ] Emit `tx.send` per vulnerability suite:
  - Injection Suite (SQLi)
  - XSS Probe Execution
  - SSRF Verification
  - LFI Parameter Path Fuzzing

## 2. Cargo & WebQ Backend
- [ ] Update `WebQ`'s `scan_api_security` Tauri command logic handling internal threading mechanisms with MPSC.

## 3. WebQ Frontend UI
- [ ] Target `src/routes/assessment/api-security/+page.svelte` for integration.
- [ ] Append logic for `ScanProgressEvent` via the generic `listen()` interface.
- [ ] Feed states cleanly into `<ScanTerminal>` representation.
- [ ] Assure output does not bloat memory upon executing heavy fuzz operations.
