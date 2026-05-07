# Bulk Domain Validator Module

## Overview
The `Bulk Domain Validator` enables security researchers to quickly audit large arrays of domains in parallel. It utilizes the `web-analyzer`'s asynchronous bulk testing routines to evaluate DNS resolution, HTTP status, and TLS certificate validity across hundreds of assets simultaneously.

## Implementation Status
**Status:** ✅ Complete

## Backend Integration
The `validate_bulk_domains` command wraps the `validate_domains_bulk` function from `web-analyzer`.
- Accepts an array of target domains.
- Fires highly concurrent (default: 10 threads) checks using `tokio::spawn`.
- Returns an aggregated `BulkValidationResult` summarizing the success and failure states of the batch.

## Frontend UI Components
Located in `/src/lib/components/recon/domain-validator/`:
*   **`ValidationDataGrid.svelte`**: A dense, sortable table displaying per-domain results for DNS, HTTP, and SSL checks. Uses Lucide icons (Check/X) to highlight failures.
*   **`ValidationStatsBar.svelte`**: A high-level dashboard indicating total success rates, active errors, and processing time.

## Missing Sub-Features
While the core module is integrated, `web-analyzer` returns significant additional data that is currently missing from the WebQ UI. See the [Intelligence Gathering Extended UI](./ideas/intelligence_gathering_extended_ui.md) request for details.

