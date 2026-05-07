# Subdomain Discovery Module

## Overview
The `Subdomain Discovery` module brings advanced asset enumeration to WebQ. It leverages the `subdomain-discovery` feature in `web-analyzer` to perform rapid, bruteforce, and dictionary-based resolution of a target's subdomains, mapping out hidden Dev, Staging, and Admin environments.

## Implementation Status
**Status:** ✅ Complete

## Backend Integration
The `scan_subdomains` Tauri command triggers `discover_subdomains`.
- Performs aggressive DNS permutations and parallel resolution.
- Pipes discovery events (e.g., "Found: dev.target.com") back to the Svelte UI in real-time via MPSC channels.
- Saves the results to the local SQLite database for historical tracking.

## Frontend UI Components
Located in `/src/lib/components/recon/subdomain-discovery/`:
*   **`SubdomainGrid.svelte`**: A robust data visualization grid charting out all resolved subdomains alongside their corresponding IP addresses and resolution times.
*   **Guides (`SubdomainTreeGuide.svelte`, etc.)**: Educational overlays detailing enumeration methodologies, the importance of asset discovery, and how to interpret the results.

## Missing Sub-Features
While the core module is integrated, `web-analyzer` returns significant additional data that is currently missing from the WebQ UI. See the [Reconnaissance Extended UI](./ideas/reconnaissance_extended_ui.md) request for details.

