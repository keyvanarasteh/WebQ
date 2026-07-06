# Web Analyzer 0.1.11 Rollout

## Status

Pending WebQ verification.

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

- `src-tauri/Cargo.toml` still uses the sibling path dependency:
  `web-analyzer = { path = "../../web-analyzer" }`.
- `src-tauri/Cargo.lock` still contains a local `web-analyzer` entry at `0.1.10`; refresh it before treating WebQ as verified against `0.1.11`.
- WebQ Tauri commands already forward progress via `app_handle.emit("scan-progress", payload)` for the newly covered modules.
- Frontend pages already render `<ScanTerminal />` for Web Technologies, Contact Spy, Advanced Content Scanner, and Bulk Domain Validator.

## Required WebQ Steps

- [ ] Decide dependency mode for the next WebQ release:
  - development mode: keep `web-analyzer = { path = "../../web-analyzer" }`;
  - release mode: switch to `web-analyzer = "0.1.11"`.
- [ ] If keeping the path dependency, run `cargo update -p web-analyzer` or `cargo check` in `src-tauri` and confirm `src-tauri/Cargo.lock` resolves the local crate as `0.1.11`.
- [ ] If switching to crates.io, update `src-tauri/Cargo.toml`, run `cargo update -p web-analyzer`, and confirm WebQ builds without the sibling repo.
- [ ] Run `cargo check` in `/Users/Q/Documents/WebQ/src-tauri`.
- [ ] Run `bun run check` at the WebQ root.
- [ ] Smoke test progress terminals for Web Technologies, Contact Spy, Advanced Content Scanner, and Bulk Domain Validator.
- [ ] Smoke test Cloudflare Bypass progress after the crate release to confirm no regression.
- [ ] Update WebQ `CHANGELOG.md` once dependency mode and verification are complete.

## Still Separate From This Rollout

- React2Shell scan/source-leak/RCE progress streaming is not currently required unless WebQ decides every long-running action must emit `scan-progress`.
- Honeypot attack/event SQLite persistence belongs to the history roadmap.
- Security Posture and Subdomain Takeover still need visible `<ScanTerminal />` rendering in the page body.
