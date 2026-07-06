# WebQ Markdown Implementation State

Last checked: 2026-07-06

## Source Markdown Reviewed

- `TODO.md`: Legacy pointer to the active todo index.
- `todo.mac.md`: Platform build checklist, not product implementation work. Keep as release reference.
- `todo.win.md`: Platform build checklist, not product implementation work. Keep as release reference.
- `REQUEST.md`: React2Shell honeypot/UI request. Backend honeypot exists, and the dashboard/signature/profile pages are now wired to live or backend data where available.
- `CHECKUP.md`: Old step-by-step audit workflow. Stale paths and process instructions; archive as scratch.
- `docs/todo/*`: Mixed old implementation plans and task files. Most progress-channel items are now implemented in backend and frontend, with a few rendering/UX gaps still active.
- `/Users/Q/Documents/web-analyzer`: Local crate source checked for function names, progress APIs, SSL expiry behavior, and release state.

## Implemented Or Mostly Implemented

- Domain Intelligence:
  - `src/lib/types/intelligence.ts` exists with typed Domain Info, DNS, SEO, and progress models.
  - Domain Info renders rich network/server/WHOIS/SSL/security details and has decoupled refresh commands.
  - Domain DNS renders records plus timestamp/response-time metadata and local history hydration.
  - SEO Analysis uses the corrected `basic_seo`, `technical_seo`, and `social_media` keys and renders all 13 major sections.
- Progress Streaming:
  - `scan-progress` MPSC forwarding exists for Domain Info, Domain DNS, SEO, Subdomain Discovery, Security Posture, Subdomain Takeover, Cloudflare Bypass, Nmap Zero Day, API Security, and Geo Analysis.
  - Most frontend pages listen to `scan-progress` and render `<ScanTerminal />`.
  - Security Posture and Subdomain Takeover now render `<ScanTerminal />` in the page body.
  - Bulk Domain Validator, Web Technologies, Contact Spy, Advanced Content, and Cloudflare Bypass now have crate-level progress support in `web-analyzer` `0.1.11`; React scan commands still do not have progress parity.
- SQLite History:
  - `src-tauri/src/db.rs` initializes SQLite with WAL mode.
  - `scans` and `scan_results` tables exist.
  - Scan logging, pagination, filters, sorting, detail blob lookup, delete, favorites, bulk delete, global stats, nuke history, unique-domain lookup, latest module hydration, and Bulk Domain Validator history logging exist.
  - `/history` and `/history/[id]` routes exist; details now show structured summary/object sections before raw JSON.
- Reporting:
  - JSON, Markdown, PDF, and DOCX export utilities exist.
- Keyboard:
  - `Ctrl/Cmd+K` command palette exists.
- SSL expiry:
  - `/Users/Q/Documents/web-analyzer/src/domain_info.rs` computes `days_until_expiry`; `web-analyzer` `0.1.11` is published and WebQ now uses the crates.io package.
- Crate contract:
  - See `docs/todo/final/web-analyzer-contract.md` for verified local function names and stale archived names.
  - WebQ currently uses crates.io `web-analyzer = "0.1.11"` for release-mode verification.
- Module integration:
  - See `docs/todo/final/module-integration-gaps.md` for progress/history parity gaps.

## Still Pending

- Native DNS clipboard/export flow.
- Central target validation across scan pages.
- `Ctrl/Cmd+Enter` scan shortcut standardization.
- F11 fullscreen and Always-on-Top controls.
- Use or remove `SpotlightCard.svelte`; currently present but not used.
- Add a real Recent Scans widget on the main dashboard.
- Add module-specific report renderers for the highest-value scan result shapes.
- Add optional SQLite persistence for honeypot attack events and attacker snapshots.
- Smoke test progress terminals in interactive Tauri dev mode.
- Final security test pass and 1.0.0 release.
