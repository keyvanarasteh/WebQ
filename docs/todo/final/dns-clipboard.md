# DNS Clipboard Export

## Status

Implemented with the browser Clipboard API used elsewhere in WebQ. Native `tauri-plugin-clipboard-manager` is deferred until there is a platform-specific need.

## Evidence

- `tauri-plugin-clipboard-manager` is not present in `package.json` or `src-tauri/Cargo.toml`.
- Some components use `navigator.clipboard`; Domain DNS now follows that existing app pattern.

## Tasks

- [x] Use existing `navigator.clipboard` app pattern instead of adding a native plugin.
- [x] Add a compact copy-JSON button to `src/routes/intelligence/domain-dns/+page.svelte`.
- [x] Serialize `DomainDnsResult.records` with domain/timestamp/response-time metadata.
- [x] Show success/failure feedback via existing toast system.
- [ ] Revisit `@tauri-apps/plugin-clipboard-manager` only if browser clipboard behavior is insufficient on a supported platform.
