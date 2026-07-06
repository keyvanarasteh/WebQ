# Release And Platform Checklists

## Status

Release preparation is mostly implemented; final verification remains.

## Implemented

- [x] Dependency check command exists for `nmap`, `dig`, and `openssl`.
- [x] Tauri bundle metadata and icons exist.
- [x] macOS and Windows build checklist markdown exists.
- [x] GitHub Actions release workflow exists.
- [x] macOS code signing failure workaround is documented in `TODO.md`.
- [x] `web-analyzer` dependency is pinned to crates.io `0.1.10`.
- [x] Local `/Users/Q/Documents/web-analyzer` source is also version `0.1.10` and contains the progress sender APIs WebQ calls.

## Remaining Tasks

- [ ] Run `bun run check`.
- [ ] Run `bun run build`.
- [ ] Make `cargo` available in this shell or document the intended Rust toolchain path.
- [ ] Run `cargo check` in `/Users/Q/Documents/web-analyzer`.
- [ ] Run `cargo check` in `src-tauri`.
- [ ] Smoke test Tauri dev mode.
- [ ] Smoke test core scans against safe test targets.
- [ ] Verify the published crates.io `web-analyzer 0.1.10` checksum/source includes the SSL expiry and progress-channel changes used by WebQ.
- [ ] Decide whether local development should use `web-analyzer = { path = "../web-analyzer" }` instead of the registry package.
- [ ] Verify macOS Apple Silicon build on the target Mac.
- [ ] Verify Windows NSIS/MSI build on a clean Windows host or VM.
- [ ] Decide 1.0.0 release criteria and update `CHANGELOG.md`.
- [ ] Tag and publish the release.
