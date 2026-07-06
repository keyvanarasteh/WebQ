# Release And Platform Checklists

## Status

Release preparation is mostly implemented; final verification remains.

## Implemented

- [x] Dependency check command exists for `nmap`, `dig`, and `openssl`.
- [x] Tauri bundle metadata and icons exist.
- [x] macOS and Windows build checklist markdown exists.
- [x] GitHub Actions release workflow exists.
- [x] macOS code signing failure workaround is documented in `TODO.md`.
- [x] WebQ uses the local sibling `web-analyzer` path dependency for active development.
- [x] Local `/Users/Q/Documents/web-analyzer` source is version `0.1.10` and contains the progress sender APIs WebQ calls.

## Remaining Tasks

- [ ] Run `bun run check`.
- [ ] Run `bun run build`.
- [ ] Make `cargo` available in this shell or document the intended Rust toolchain path.
- [ ] Run `cargo check` in `/Users/Q/Documents/web-analyzer`.
- [ ] Run `cargo check` in `src-tauri`.
- [ ] Smoke test Tauri dev mode.
- [ ] Smoke test core scans against safe test targets.
- [ ] Publish or version-bump `web-analyzer` before switching WebQ back to a crates.io dependency for release.
- [ ] Verify macOS Apple Silicon build on the target Mac.
- [ ] Verify Windows NSIS/MSI build on a clean Windows host or VM.
- [ ] Decide 1.0.0 release criteria and update `CHANGELOG.md`.
- [ ] Tag and publish the release.
