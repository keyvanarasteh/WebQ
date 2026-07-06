# macOS Release Signing Audit

## Status

Pending. Current macOS release verification failed before Tauri reached the native build/sign/notarize phase.

## Latest Check

- Workflow: `Release`
- Run: `28827615592`
- Commit: `39f3435`
- Trigger: manual `workflow_dispatch`
- Result: failed

## What Passed

- Apple release secrets exist in repository Actions secrets.
- `Derive Apple signing identity` passed for:
  - `aarch64-apple-darwin`
  - `x86_64-apple-darwin`
- This means the previous certificate/identity mismatch is no longer the active blocker.

## Active Blocker

`tauri-apps/tauri-action@v0` still auto-detects and runs Bun:

```text
running bun [ 'tauri', 'build', '--target', 'aarch64-apple-darwin' ]
running bun [ 'tauri', 'build', '--target', 'x86_64-apple-darwin' ]
```

The workflow now installs Node/npm, not Bun. The repository still contains both `bun.lock` and `package-lock.json`, so the action chooses Bun from the lockfile state.

Observed failures:

- macOS jobs fail immediately after Bun starts, with an unsettled top-level-await warning from the action bundle.
- Windows fails because `bun` is not installed.
- Linux fails in the same action path before a real Tauri build begins.

## Audit And Fix Tasks

- [ ] Force the Tauri action to use npm by adding `tauriScript: npm run tauri` to `.github/workflows/release.yml`.
- [ ] Decide whether `bun.lock` should stay in the repo or be removed if release builds are standardized on npm.
- [ ] Re-run `Release` manually from `main`.
- [ ] Confirm both macOS jobs pass `Derive Apple signing identity`.
- [ ] Confirm both macOS jobs reach Rust/Tauri compilation.
- [ ] Confirm both macOS jobs produce `.dmg` artifacts.
- [ ] Confirm signing succeeds with the derived identity.
- [ ] Confirm notarization succeeds using `APPLE_ID`, `APPLE_PASSWORD`, and `APPLE_TEAM_ID`.
- [ ] Verify resulting DMGs on Apple Silicon and Intel macOS where possible.

## Notes

- The current failure does not prove Apple notarization credentials are bad; the jobs stop before notarization.
- `tauri-action` documents `tauriScript` as the supported way to override the package-manager command. It must not include `build` or target args; the action appends those itself.
