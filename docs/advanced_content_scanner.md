# Advanced Content Scanner Module

## Overview
The `Advanced Content Scanner` aggressively sweeps a target environment for hardcoded secrets, misconfigured endpoints (e.g., exposed `/env`, `/config`), and API key leakage. 

## Implementation Status
**Status:** ✅ Complete

## Backend Integration
The `scan_advanced_content` command utilizes the `advanced_content_scanner` from `web-analyzer`.
- Checks common exposed directories.
- Analyzes Javascript bundles and HTML source for API key patterns (AWS, Stripe, GitHub tokens, etc.).
- Categorizes findings by severity levels (Critical, High, Medium, Low).

## Frontend UI Components
Located in `/src/lib/components/recon/advanced-scanner/`:
*   **`ScannerMasonry.svelte`**: A dense, masonry-style grid layout grouping the detected vulnerabilities, exposed endpoints, and secret leakages by severity.
*   **`AdvancedScannerGuide.svelte`**: A comprehensive educational guide explaining the impact of secret exposure and mitigation techniques.
