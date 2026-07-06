# Snapcraft Release Audit

## Status

Pending. Snap packaging is intentionally deferred while macOS release signing is checked.

## Current State

- `SNAPCRAFT_STORE_CREDENTIALS` exists in repository Actions secrets.
- The Snap workflow is manual/tag-triggered only: `.github/workflows/snap.yml`.
- The latest manual run was cancelled intentionally:
  - Run: `28827040561`
  - Commit: `cbbdc3a`
  - Reason: Snapcraft was deferred for now.
- `snap/snapcraft.yaml` currently builds with `cargo tauri build --no-bundle` so Snapcraft, not Tauri, owns Snap packaging.

## Known Failures Already Addressed

- `snapcraft --target-arch` was invalid for Snapcraft 9.
- `arm64` build planning failed because the project only allowed `build-on: arm64`.
- Bun failed during package install because the inlang/Paraglide plugin import used jsDelivr `latest` and generated a very long data URL in the Snapcraft build image.
- Tauri Linux bundling failed because `/usr/bin/xdg-open` was missing while Tauri tried to create `.deb`, `.rpm`, and `.AppImage` artifacts inside Snapcraft.

## Audit Tasks

- [ ] Re-run `Publish multi-arch Snap` manually from `main`.
- [ ] Verify `amd64` reaches `Validate Snap Store credentials`.
- [ ] Verify `arm64` reaches `Validate Snap Store credentials`.
- [ ] If either architecture stalls in `Build Snap`, inspect completed logs for the exact Snapcraft phase.
- [ ] Confirm `cargo tauri build --no-bundle` still creates `src-tauri/target/release/webq-app` for both architectures.
- [ ] Confirm the final Snap contains `/usr/bin/webq`.
- [ ] Confirm strict confinement plugs cover WebQ runtime needs: `network`, `network-bind`, and `home`.
- [ ] Smoke test the installed Snap on Linux with DNS, HTTP, and scanner dependency checks.
- [ ] Publish to Snap Store `edge`.
- [ ] Promote from `edge` only after a Linux smoke test passes.

## Notes

- Keep Snap release work separate from the standard Tauri release workflow.
- Do not reintroduce Bun into Snapcraft unless the Paraglide/inlang install path is proven stable in the Snapcraft image.
