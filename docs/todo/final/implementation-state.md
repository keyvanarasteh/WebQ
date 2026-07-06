# WebQ Markdown Implementation State

Last checked: 2026-07-06

## Source Markdown Reviewed

- `TODO.md`: Main roadmap. Mostly accurate for early modules, but several Phase 7 SQLite/history items are now partially implemented and should be split into smaller remaining tasks.
- `todo.mac.md`: Platform build checklist, not product implementation work. Keep as release reference.
- `todo.win.md`: Platform build checklist, not product implementation work. Keep as release reference.
- `REQUEST.md`: React2Shell honeypot/UI request. Backend honeypot exists, but dashboard/signature/profile pages are not fully wired to live data.
- `CHECKUP.md`: Old step-by-step audit workflow. Stale paths and process instructions; archive as scratch.
- `docs/todo/*`: Mixed old implementation plans and task files. Most progress-channel items are now implemented in backend and frontend, with a few rendering/UX gaps still active.
- `/Users/Q/Documents/web-analyzer`: Local crate source checked for function names, progress APIs, and SSL expiry behavior.

## Implemented Or Mostly Implemented

- Domain Intelligence:
  - `src/lib/types/intelligence.ts` exists with typed Domain Info, DNS, SEO, and progress models.
  - Domain Info renders rich network/server/WHOIS/SSL/security details and has decoupled refresh commands.
  - Domain DNS renders records plus timestamp/response-time metadata and local history hydration.
  - SEO Analysis uses the corrected `basic_seo`, `technical_seo`, and `social_media` keys and renders all 13 major sections.
- Progress Streaming:
  - `scan-progress` MPSC forwarding exists for Domain Info, Domain DNS, SEO, Subdomain Discovery, Security Posture, Subdomain Takeover, Cloudflare Bypass, Nmap Zero Day, API Security, and Geo Analysis.
  - Most frontend pages listen to `scan-progress` and render `<ScanTerminal />`.
  - Security Posture and Subdomain Takeover listen to `scan-progress` but still do not render `<ScanTerminal />` in the page body.
  - Bulk Domain Validator now has start/complete progress events; Web Technologies, Contact Spy, Advanced Content, and React scan commands do not have progress parity yet.
- SQLite History:
  - `src-tauri/src/db.rs` initializes SQLite with WAL mode.
  - `scans` and `scan_results` tables exist.
  - Scan logging, pagination, detail blob lookup, delete, stats, nuke history, unique-domain lookup, latest module hydration, and Bulk Domain Validator history logging exist.
  - `/history` and `/history/[id]` routes exist.
- Reporting:
  - JSON, Markdown, PDF, and DOCX export utilities exist.
- Keyboard:
  - `Ctrl/Cmd+K` command palette exists.
- SSL expiry:
  - `/Users/Q/Documents/web-analyzer/src/domain_info.rs` computes `days_until_expiry`; WebQ uses the local path crate until those changes are published or version-bumped.
- Crate contract:
  - See `docs/todo/final/web-analyzer-contract.md` for verified local function names and stale archived names.
  - WebQ currently uses the sibling local path dependency at `/Users/Q/Documents/web-analyzer` for active development.
- Module integration:
  - See `docs/todo/final/module-integration-gaps.md` for progress/history parity gaps.

## Still Pending

- Native DNS clipboard/export flow.
- Central target validation across scan pages.
- `Ctrl/Cmd+Enter` scan shortcut standardization.
- F11 fullscreen and Always-on-Top controls.
- Use or remove `SpotlightCard.svelte`; currently present but not used.
- Finish history filtering, favorites, bulk actions, statistics, and richer detail hydration.
- Render progress terminals in Security Posture and Subdomain Takeover pages; they listen but do not display the terminal.
- Decide whether React2Shell scan/source-leak/RCE commands need `scan-progress` support.
- React2Shell live dashboard/profile/signature completion.
- Publish or version-bump `web-analyzer` before returning WebQ to a crates.io release dependency.
- Re-run Rust checks once `cargo` is available in PATH.
- Final security test pass and 1.0.0 release.
