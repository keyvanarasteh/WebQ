# WebQ Todo Index

Use `docs/todo/final/` as the active todo source. The list below is ordered for a clean DevOps path: verify the dependency contract first, roll WebQ onto the released engine state, close integration gaps, stabilize UX/data, then package and release.

## Current Engine Baseline

- `web-analyzer` `0.1.11` is published on crates.io and tagged as `v0.1.11`.
- The release includes progress sender support for Web Technologies, Contact Spy, Advanced Content Scanner, Cloudflare Bypass, and Bulk Domain Validator per-domain progress.
- WebQ still needs a dependency-mode decision before release: keep the sibling path dependency for development, or switch `src-tauri/Cargo.toml` to `web-analyzer = "0.1.11"` for crates.io-only release builds.
- `src-tauri/Cargo.lock` should be refreshed and checked because it may still record the local path crate as `0.1.10`.

## DevOps Order

| Order | Todo | Status |
|---:|---|---|
| 1 | [Implementation State](final/implementation-state.md) | done |
| 2 | [Web Analyzer Contract](final/web-analyzer-contract.md) | done |
| 3 | [Web Analyzer 0.1.11 Rollout](final/web-analyzer-0-1-11-rollout.md) | pending |
| 4 | [Module Integration Gaps](final/module-integration-gaps.md) | in progress |
| 5 | [DNS Clipboard Export](final/dns-clipboard.md) | done |
| 6 | [Scan History And SQLite](final/history-sqlite.md) | in progress |
| 7 | [React2Shell Honeypot And UI](final/react2shell.md) | in progress |
| 8 | [UX Polish And Shortcuts](final/ux-polish.md) | in progress |
| 9 | [Release And Platform Checklists](final/release.md) | pending |
| 10 | [macOS Release Checklist](final/release-macos.md) | pending |
| 11 | [Windows Release Checklist](final/release-windows.md) | pending |
| 12 | [Cleanup And Trash Notes](final/cleanup-trash.md) | pending |

## Next WebQ Implementation Steps

- Refresh `src-tauri/Cargo.lock` against `web-analyzer` `0.1.11`.
- Choose whether WebQ release builds should use the sibling path dependency or crates.io `web-analyzer = "0.1.11"`.
- Run `cargo check` in `src-tauri` and `bun run check` at the WebQ root.
- Smoke test progress terminals for Web Technologies, Contact Spy, Advanced Content Scanner, Bulk Domain Validator, and Cloudflare Bypass.
- Render the already-wired progress terminals for Security Posture and Subdomain Takeover.
- Decide whether React2Shell scan/source-leak/RCE commands should emit `scan-progress`, or document them as final-result-only actions.
