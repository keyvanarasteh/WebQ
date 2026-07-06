# DNS Clipboard Export

## Status

Not implemented as native Tauri clipboard-manager flow.

## Evidence

- `TODO.md` still lists Domain DNS JSON clipboard export as open.
- `tauri-plugin-clipboard-manager` is not present in `package.json` or `src-tauri/Cargo.toml`.
- Some components use `navigator.clipboard`, but Domain DNS does not provide the requested full-record JSON copy action.

## Tasks

- [ ] Add `@tauri-apps/plugin-clipboard-manager` and Rust plugin dependency if native clipboard support is preferred.
- [ ] Register the clipboard plugin in `src-tauri/src/lib.rs`.
- [ ] Add capability permissions if required by Tauri v2.
- [ ] Add a compact copy-JSON button to `src/routes/intelligence/domain-dns/+page.svelte`.
- [ ] Serialize `DomainDnsResult.records` with domain/timestamp metadata.
- [ ] Show success/failure feedback via existing toast or inline status.
- [ ] Keep `navigator.clipboard` as a browser-safe fallback if the native plugin is unavailable.

