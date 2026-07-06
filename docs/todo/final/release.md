# Release And Platform Checklists

## Status

Release preparation is mostly implemented; final verification remains.

## Implemented

- [x] Dependency check command exists for `nmap`, `dig`, and `openssl`.
- [x] Tauri bundle metadata and icons exist.
- [x] macOS and Windows build checklist markdown exists.
- [x] GitHub Actions release workflow exists.
- [x] macOS code signing failure workaround is documented in `TODO.md`.
- [x] WebQ uses crates.io `web-analyzer = "0.1.11"` for release-mode verification.
- [x] Local `/Users/Q/Documents/web-analyzer` source is version `0.1.11` and contains the progress sender APIs WebQ calls.
- [x] `web-analyzer` `0.1.11` is published on crates.io and tagged `v0.1.11`.
- [x] `web-analyzer` release CI and publish workflow passed.
- [x] `src-tauri/Cargo.lock` resolves the crates.io `web-analyzer` `0.1.11` package.

## Remaining Tasks

- [x] Decide whether WebQ release builds should use `web-analyzer = { path = "../../web-analyzer" }` or crates.io `web-analyzer = "0.1.11"`.
- [x] Refresh `src-tauri/Cargo.lock` so it no longer records stale local `web-analyzer` `0.1.10` metadata.
- [x] Run `npm run check`.
- [x] Run `npm run build`.
- [x] Run `cargo check` in `/Users/Q/Documents/web-analyzer`.
- [x] Run `cargo check` in `src-tauri`.
- [ ] Smoke test Tauri dev mode.
- [ ] Smoke test core scans against safe test targets, including Web Technologies, Contact Spy, Advanced Content Scanner, Bulk Domain Validator, and Cloudflare Bypass progress.
- [ ] Verify macOS Apple Silicon build on the target Mac.
- [ ] Verify Windows NSIS/MSI build on a clean Windows host or VM.
- [ ] Re-run and audit Snapcraft packaging; see [Snapcraft Release Audit](snapcraft-release-audit.md).
- [ ] Decide 1.0.0 release criteria and update `CHANGELOG.md`.
- [ ] Tag and publish the release.
