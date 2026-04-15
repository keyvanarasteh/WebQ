# Subdomain Takeover (Phase 4.2) Tasklist

## 1. Web-Analyzer Core Refactor
- [ ] Update `web_analyzer::subdomain_takeover::verify_takeovers` parameter to accept `Option<tokio::sync::mpsc::Sender<ScanProgress>>`.
- [ ] Inject initial `tx.send` progress tick denoting the loading of 36 CNAME signature templates (AWS, GitHub, Azure, etc).
- [ ] Inject continuous progress ticks inside the scanning loop mapping each subdomain verification.

## 2. Cargo & WebQ Backend
- [ ] Ensure `web-analyzer` tests for subdomain module pass.
- [ ] Modify `src-tauri/src/lib.rs` hook (`scan_subdomain_takeover`) to spawn MPSC channel to `app_handle.emit("scan-progress")`.

## 3. WebQ Frontend UI
- [ ] Open `src/routes/assessment/subdomain-takeover/+page.svelte`.
- [ ] Introduce `listen("scan-progress")` binding to state array.
- [ ] Provide conditionally rendered `<ScanTerminal logs={...} />` into the DOM.
- [ ] Verify that realtime output accurately parses through the terminal during live CNAME querying.
