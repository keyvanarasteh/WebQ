# Web Analyzer Contract Check

Last checked: 2026-07-06 against `/Users/Q/Documents/web-analyzer` and crates.io `web-analyzer` `0.1.11`.

## Dependency Reality

- Local crate path: `/Users/Q/Documents/web-analyzer`
- Local crate version: `0.1.11`
- Published crate version: `0.1.11`
- Published tag: `v0.1.11`
- WebQ dependency declaration: `web-analyzer = "0.1.11"` in `src-tauri/Cargo.toml`
- WebQ lockfile source: crates.io registry entry with checksum `0c6607e9fee3ee048b3d31937257cf839a75bf53f83a45b3430a489e46b941dc`

Important: WebQ is now wired to the published crate for release-mode verification. Switch back to the sibling path dependency only for active local crate development.

## Progress API Contract

| WebQ command | Local crate function | Progress sender | Status |
|---|---|---:|---|
| `scan_domain_info` | `domain_info::get_domain_info(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_domain_dns` | `domain_dns::get_dns_records(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_seo_analysis` | `seo_analysis::analyze_advanced_seo(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_web_technologies` | `web_technologies::detect_web_technologies(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `validate_bulk_domains` | `domain_validator::validate_domains_bulk(&[String], usize, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_subdomains` | `subdomain_discovery::discover_subdomains(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_contacts` | `contact_spy::crawl_contacts(&str, usize, Option<Sender<ScanProgress>>)` | Yes | Matches |
| `scan_advanced_content` | `advanced_content_scanner::scan_content(&str, Option<Sender<ScanProgress>>)` | Yes | Matches |
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
- `web-analyzer` `0.1.11` includes the progress sender support WebQ now calls for Web Technologies, Contact Spy, Advanced Content Scanner, Cloudflare Bypass, and Bulk Domain Validator per-domain progress.
- `web-analyzer` `0.1.11` includes the Android/Rustls CI fix that removed the AWS-LC/Android compiler blocker.

## Remaining Verification Tasks

- [x] Run `cargo check` in `/Users/Q/Documents/web-analyzer`.
- [x] Publish or version-bump `web-analyzer` before switching WebQ back to a crates.io release dependency.
- [x] Refresh `src-tauri/Cargo.lock` so WebQ resolves crates.io `web-analyzer = "0.1.11"`.
- [x] Run `cargo check` in `/Users/Q/Documents/WebQ/src-tauri`.
- [x] Run `npm run check` in `/Users/Q/Documents/WebQ`.
