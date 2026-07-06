# Web Analyzer Contract Check

Last checked: 2026-07-06 against `/Users/Q/Documents/web-analyzer`.

## Dependency Reality

- Local crate path: `/Users/Q/Documents/web-analyzer`
- Local crate version: `0.1.10`
- WebQ dependency declaration: `web-analyzer = "0.1.10"` in `src-tauri/Cargo.toml`
- WebQ lockfile source: `registry+https://github.com/rust-lang/crates.io-index`
- WebQ lockfile checksum: `65e3a5d12b8c0fdbef8d362a009162334836a455e254270d157bdcec2cb33916`

Important: WebQ is not currently wired to the local path crate. The local source can match the published `0.1.10` crate, but that was not proven here because `cargo` is unavailable in this shell.

## Progress API Contract

| WebQ command | Local crate function | Progress sender | Status |
|---|---|---:|---|
| `scan_domain_info` | `domain_info::get_domain_info(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_domain_dns` | `domain_dns::get_dns_records(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_seo_analysis` | `seo_analysis::analyze_advanced_seo(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_subdomains` | `subdomain_discovery::discover_subdomains(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_security_posture` | `security_analysis::analyze_security(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_subdomain_takeover` | `subdomain_takeover::check_subdomain_takeover(&str, &[String], Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_cloudflare_bypass` | `cloudflare_bypass::find_real_ip(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_nmap_zero_day` | `nmap_zero_day::run_nmap_scan(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_api_security` | `api_security_scanner::scan_api_endpoints(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_geo_analysis` | `geo_analysis::analyze_geo(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |

## Old Task Names Corrected

Archived scratch docs used these stale or wrong names:

- `cloudflare_bypass::execute_bypass` should be `cloudflare_bypass::find_real_ip`.
- `nmap_zero_day::run_cve_scan` should be `nmap_zero_day::run_nmap_scan`.
- `subdomain_takeover::verify_takeovers` should be `subdomain_takeover::check_subdomain_takeover`.
- `geo_analysis::analyze_geometry` should be `geo_analysis::analyze_geo`.
- `api_security_scanner::fuzz_endpoints` is not the WebQ command boundary; WebQ calls `scan_api_endpoints`.

## Local Crate Findings

- `ScanProgress` is defined in `src/lib.rs` with `module`, `percentage`, `message`, and `status`.
- `domain_info::check_ssl` computes `days_until_expiry` from OpenSSL `notAfter` output in the local source.
- `react_honeypot::HoneypotEngine` provides `process_request`, `get_top_threats`, and config support used by WebQ.

## Remaining Verification Tasks

- [ ] Make `cargo` available in this shell.
- [ ] Run `cargo check` in `/Users/Q/Documents/web-analyzer`.
- [ ] Run `cargo check` in `/Users/Q/Documents/WebQ/src-tauri`.
- [ ] Verify the crates.io `web-analyzer 0.1.10` package source matches the local source for progress APIs and SSL expiry.
- [ ] Decide whether WebQ should keep using the registry package or use a local path dependency during development.

