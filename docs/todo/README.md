# WebQ Todo Index

Use `docs/todo/final/` as the active todo source. The list below is ordered for a clean DevOps path: verify the dependency contract first, roll WebQ onto the released engine state, close integration gaps, stabilize UX/data, then package and release.

## Current Engine Baseline

- `web-analyzer` `0.1.11` is published on crates.io and tagged as `v0.1.11`.
- The release includes progress sender support for Web Technologies, Contact Spy, Advanced Content Scanner, Cloudflare Bypass, and Bulk Domain Validator per-domain progress.
- WebQ now uses the published crates.io dependency: `web-analyzer = "0.1.11"`.
- `src-tauri/Cargo.lock` is refreshed against the registry crate and includes the crates.io checksum.
- Local verification passed with `npm run check`, `npm run build`, and `cargo check` in `src-tauri`.

## DevOps Order

| Order | Todo | Status |
|---:|---|---|
| 1 | [Implementation State](final/implementation-state.md) | done |
| 2 | [Web Analyzer Contract](final/web-analyzer-contract.md) | done |
| 3 | [Web Analyzer 0.1.11 Rollout](final/web-analyzer-0-1-11-rollout.md) | done |
| 4 | [Module Integration Gaps](final/module-integration-gaps.md) | in progress |
| 5 | [DNS Clipboard Export](final/dns-clipboard.md) | done |
| 6 | [Scan History And SQLite](final/history-sqlite.md) | in progress |
| 7 | [React2Shell Honeypot And UI](final/react2shell.md) | in progress |
| 8 | [UX Polish And Shortcuts](final/ux-polish.md) | in progress |
| 9 | [Release And Platform Checklists](final/release.md) | pending |
| 10 | [macOS Release Signing Audit](final/macos-release-signing-audit.md) | pending |
| 11 | [macOS Release Checklist](final/release-macos.md) | pending |
| 12 | [Windows Release Checklist](final/release-windows.md) | pending |
| 13 | [Snapcraft Release Audit](final/snapcraft-release-audit.md) | pending |
| 14 | [Cleanup And Trash Notes](final/cleanup-trash.md) | pending |

## Next WebQ Implementation Steps

- Smoke test progress terminals in Tauri dev mode for Web Technologies, Contact Spy, Advanced Content Scanner, Bulk Domain Validator, Cloudflare Bypass, Security Posture, and Subdomain Takeover.
- Add optional SQLite persistence for honeypot attack events and attacker snapshots if historical analysis is required.
- Add a real Recent Scans widget on the dashboard.
- Fix the release workflow package-manager mismatch so `tauri-action` runs `npm run tauri` instead of auto-detected Bun.
- Finish release packaging checks for macOS and Windows.
- Re-run and audit Snapcraft after macOS signing is green.
