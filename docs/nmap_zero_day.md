# Nmap Zero-Day Module

## Overview
The `Nmap Zero-Day` module acts as a powerful local integration of the `nmap` port scanner. It correlates discovered services and versions against the National Vulnerability Database (NVD) to automatically map CVEs to open ports.

## Implementation Status
**Status:** ✅ Complete

## Backend Integration
The `scan_nmap_zero_day` command invokes `run_nmap_scan`.
- Executes the local `nmap` binary (which is verified via `check_dependencies`).
- Uses specific scripts (e.g., `vulners`) to map service fingerprints to CVEs.
- Parses the XML/JSON output of Nmap natively into Rust structs.

## Frontend UI Components
Located in `/src/lib/components/assessment/nmap-zero-day/`:
*   **`NmapPortsGrid.svelte`**: Maps open ports, states, and the specific service/daemon running on them.
*   **`NmapVulnGrid.svelte`**: A critical matrix displaying CVE identifiers, CVSS scores, and descriptions mapped directly to the services found in the port scan.
*   **`DnsInfoGrid.svelte`**: Correlates the underlying DNS infrastructure related to the scanned IP.
