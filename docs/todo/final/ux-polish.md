# UX Polish And Shortcuts

## Status

Partially implemented.

## Implemented

- [x] Active page highlighting exists in the sidebar.
- [x] Roadmap page includes segmented HTTP status/error matrix work.
- [x] `Ctrl/Cmd+K` command palette exists.
- [x] Per-input Enter-to-scan exists on many pages.
- [x] `SpotlightCard.svelte` exists as a reusable component.

## Remaining Tasks

- [ ] Decide whether to integrate `SpotlightCard.svelte` across repeated cards or remove it from the TODO as already sufficient.
- [ ] Add central target/domain/URL validation helper and reuse across scan pages.
- [ ] Standardize validation messages and disabled scan states.
- [ ] Add `Ctrl/Cmd+Enter` to trigger the primary scan action on scan pages.
- [ ] Add F11 fullscreen toggle via Tauri window APIs.
- [ ] Add Always-on-Top toggle via Tauri window APIs.
- [ ] Update Tauri permissions/capabilities for fullscreen and always-on-top if needed.
- [ ] Render `<ScanTerminal />` during Security Posture scans; the listener already exists.
- [ ] Render `<ScanTerminal />` during Subdomain Takeover scans; the listener already exists.
- [ ] Remove duplicated `logs = []; progressPercent = 0;` assignments in assessment pages.
- [ ] Replace broad `any` types in assessment page result objects with typed interfaces.
- [ ] Use toast feedback consistently for scan failures and successful DB saves.
