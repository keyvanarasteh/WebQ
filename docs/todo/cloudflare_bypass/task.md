# Cloudflare Bypass (Phase 4.3) Tasklist

## 1. Web-Analyzer Core Refactor
- [ ] Open `web_analyzer::cloudflare_bypass::execute_bypass` and wire `Option<tokio::sync::mpsc::Sender<ScanProgress>>`.
- [ ] Inform iteration over DNS Lookup Histories using `tx.send`.
- [ ] Report TCP verification handshakes directly to UI using real-time channels.

## 2. Cargo & WebQ Backend
- [ ] Run `cargo check` and modify `src-tauri/src/lib.rs` endpoint `scan_cloudflare_bypass`.
- [ ] Map internal channel structure to `tauri_app` emit stream.

## 3. WebQ Frontend UI
- [ ] Connect `src/routes/assessment/cloudflare-bypass/+page.svelte`.
- [ ] Use `listen` across `ScanTerminal.svelte` properties (`$state` logs, percentages).
- [ ] Check auto-scroll interactions during live IP enumeration iterations.
