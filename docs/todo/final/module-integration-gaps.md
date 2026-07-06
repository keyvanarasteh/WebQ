# Module Integration Gaps

Last checked: 2026-07-06 against WebQ and `/Users/Q/Documents/web-analyzer`.

## Status

Several modules are functional but do not have the same progress/history integration level as the main intelligence and assessment pages.

## Progress Streaming Coverage

Implemented `scan-progress` streaming:

- [x] Domain Info
- [x] Domain DNS
- [x] SEO Analysis
- [x] Subdomain Discovery
- [x] Security Posture
- [x] Subdomain Takeover
- [x] Cloudflare Bypass
- [x] Nmap Zero Day
- [x] API Security
- [x] Geo Analysis

No `scan-progress` streaming currently wired:

- [ ] Web Technologies: `detect_web_technologies(&url)` has no progress sender in the local crate.
- [ ] Bulk Domain Validator: `validate_domains_bulk(&domains, 10)` has no WebQ progress channel wrapper.
- [ ] Contact Spy: `crawl_contacts(&domain, max_pages)` has no WebQ progress channel wrapper.
- [ ] Advanced Content Scanner: `scan_content(&domain)` has no WebQ progress channel wrapper.
- [ ] React2Shell scanner/source-leak/RCE commands do not emit `scan-progress`.

## History Logging Coverage

Logged through `log_and_execute_scan!`:

- [x] Domain Info
- [x] Domain DNS
- [x] SEO Analysis
- [x] Web Technologies
- [x] Subdomain Discovery
- [x] Contact Spy
- [x] Advanced Content
- [x] Security Posture
- [x] Subdomain Takeover
- [x] Cloudflare Bypass
- [x] Nmap Zero Day
- [x] API Security
- [x] Geo Analysis
- [x] React source leak/RCE/React2Shell scanner commands

Not logged:

- [ ] Bulk Domain Validator: `validate_bulk_domains(domains)` does not accept the SQLite pool and does not call `log_and_execute_scan!`.
- [ ] Honeypot attack events are not persisted to SQLite history.

## Tasks

- [ ] Decide whether every module needs progress streaming, or document which modules intentionally return only final results.
- [ ] Add progress sender support in `web-analyzer` for Web Technologies, Contact Spy, Advanced Content, and Bulk Domain Validator if live terminal parity is desired.
- [ ] Update WebQ Tauri commands for those modules to forward progress via `app_handle.emit("scan-progress", payload)`.
- [ ] Add SQLite logging for Bulk Domain Validator.
- [ ] Add optional SQLite persistence for honeypot events and attacker profiles.
- [ ] Update `TODO.md` to stop marking Bulk Domain Validator streaming as complete until it is actually wired.

