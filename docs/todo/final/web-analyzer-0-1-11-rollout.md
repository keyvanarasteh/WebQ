# Web Analyzer 0.1.11 Rollout

## Status

Implemented in WebQ. Interactive scan smoke tests remain in the release checklist.

## Release Facts

- `web-analyzer` `0.1.11` is published on crates.io.
- Git tag `v0.1.11` exists in `keyvanarasteh/web-analyzer`.
- Release workflow passed and published successfully.
- CI passed for the release commit, including check, test, clippy, format, and documentation.

## Implemented In `web-analyzer` 0.1.11

- [x] `ScanProgress` sender support for Web Technologies.
- [x] `ScanProgress` sender support for Contact Spy.
- [x] `ScanProgress` sender support for Advanced Content Scanner.
- [x] Per-domain `ScanProgress` sender support for Bulk Domain Validator.
- [x] Progress sender coverage retained for Cloudflare Bypass and the existing scan modules.
- [x] Shared `http_client_builder()` for `reqwest` clients and Rustls provider initialization.
- [x] Android target compatibility for `cargo check --all-features --target aarch64-linux-android` without requiring an Android NDK compiler.
- [x] CI cleanup for format, clippy, and integration-test stability.
- [x] Docs index and changelog updated in the crate.

## Current WebQ State

- `src-tauri/Cargo.toml` uses the published crate:
  `web-analyzer = "0.1.11"`.
- `src-tauri/Cargo.lock` resolves `web-analyzer` `0.1.11` from crates.io with checksum `0c6607e9fee3ee048b3d31937257cf839a75bf53f83a45b3430a489e46b941dc`.
- WebQ Tauri commands already forward progress via `app_handle.emit("scan-progress", payload)` for the newly covered modules.
- Frontend pages already render `<ScanTerminal />` for Web Technologies, Contact Spy, Advanced Content Scanner, and Bulk Domain Validator.
- Security Posture and Subdomain Takeover now also render their already-wired `<ScanTerminal />` components.

## Required WebQ Steps

- [x] Decide dependency mode for the next WebQ release: use crates.io `web-analyzer = "0.1.11"`.
- [x] Switch `src-tauri/Cargo.toml` to crates.io and refresh `src-tauri/Cargo.lock`.
- [x] Run `cargo check` in `/Users/Q/Documents/WebQ/src-tauri`.
- [x] Run `npm run check` at the WebQ root.
- [x] Run `npm run build` at the WebQ root.
- [x] Update WebQ `CHANGELOG.md` once dependency mode and verification are complete.
- [ ] Smoke test progress terminals for Web Technologies, Contact Spy, Advanced Content Scanner, and Bulk Domain Validator.
- [ ] Smoke test Cloudflare Bypass progress after the crate release to confirm no regression.

## Still Separate From This Rollout

- React2Shell scan/source-leak/RCE progress streaming is not currently required unless WebQ decides every long-running action must emit `scan-progress`.
- Honeypot attack/event SQLite persistence belongs to the history roadmap.
- Platform packaging and interactive Tauri smoke tests belong to the release roadmap.
