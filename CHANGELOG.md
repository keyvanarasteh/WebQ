# Changelog

All notable changes to the **WebQ** project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [Unreleased]

### Added
- **React2Shell Honeypot Telemetry UI** — Telemetry dashboard with KPI counters, live threat feed, and attack vector distribution visualization. (PR [#7](https://github.com/keyvanarasteh/WebQ/pull/7) by [@mrrtzz](https://github.com/mrrtzz))
- **Signature Matrix Page** — Comprehensive documentation of honeypot detection vectors, example payloads, and recommended remediations.
- **Attacker Profiling Page** — Top attacker statistics, risk scores, geographic origin mapping, and browser fingerprint analysis.
- **Contributors Section** — Added Contributors panel to the About page.
- **Scan History Controls** — Added history filters, global statistics, favorites, bulk delete, and structured scan detail summaries.
- **macOS Tauri Capability Schema** — Added the generated macOS capability schema emitted by Tauri validation.

### Changed
- Pivoted React route page from exploit execution controls to a read-only analytics and monitoring experience.
- Added TypeScript typing to the `CrawlingConsole` module script for improved type safety.
- Switched the Tauri backend to the published `web-analyzer = "0.1.11"` crate and refreshed the lockfile against crates.io.
- Wired Security Posture and Subdomain Takeover pages to render their existing scan progress terminals.
- Replaced React honeypot mock data with live Tauri events and backend attacker profile data.

### Fixed
- Corrected the React honeypot local payload tester invocation to match the backend command contract.

### Contributors
- **Morteza (Mori)** — [@mrrtzz](https://github.com/mrrtzz) — 15-year coding partner and cybersecurity collaborator. First official contributor to the WebQ platform.
