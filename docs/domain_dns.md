# Domain DNS Module

## Overview
The `Domain DNS` module integrates the deep DNS querying capabilities of the `web-analyzer` crate directly into the WebQ application. It performs exhaustive DNS record lookups (A, AAAA, MX, TXT, NS, CAA) and analyzes the underlying security parameters of the domain's routing configuration.

## Implementation Status
**Status:** ✅ Complete

The backend properly maps `web-analyzer`'s DNS concurrent queries, and the frontend consumes these via tailored Svelte 5 visualization components.

## Backend Integration
The Tauri backend provides the `scan_domain_dns` and `scan_dns_records` IPC commands. 
- These commands spawn asynchronous resolution channels using `tokio` to fetch all available DNS record types without blocking the frontend.
- Supports progressive scanning and broadcasts updates to the UI using Tauri's MPSC event emitters.

## Frontend UI Components
The module relies on the following Svelte 5 components (located in `/src/lib/components/intelligence/domain-dns/`):

*   **`DnsRecordsBoard.svelte`**: A dynamic data grid that maps out all detected DNS records (A, AAAA, CNAME, MX, TXT, etc.). It organizes the output with distinct icons and categories for readability.
*   **`DnsSecurityCheck.svelte`**: Evaluates specific DNS records (like SPF, DMARC in TXT records, and CAA restrictions) to visualize the domain's email spoofing and certificate issuance protection posture.
*   **Guide Components**: Interactive tutorial overlays (`DnsRecordsBoardGuide.svelte` & `DnsSecurityCheckGuide.svelte`) provide on-page context and OSINT training for analysts.

## Data Flow
1.  The target domain is entered in `/intelligence/domain-dns`.
2.  Tauri fires the `scan_domain_dns` command, delegating to the `web-analyzer` crate.
3.  The backend streams realtime DNS lookup events to the frontend.
4.  The results are populated into the reactive `DnsRecordsBoard` and analyzed by the `DnsSecurityCheck` module to assess threat vulnerability levels.

## Missing Sub-Features
While the core module is integrated, `web-analyzer` returns significant additional data that is currently missing from the WebQ UI. See the [Intelligence Gathering Extended UI](./ideas/intelligence_gathering_extended_ui.md) request for details.

