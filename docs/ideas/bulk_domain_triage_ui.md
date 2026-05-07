# Feature Request: High-Concurrency Bulk Domain Triage UI

## Overview
Currently, the WebQ frontend only allows an analyst to scan a single target domain at a time. However, the `web-analyzer` Rust backend contains a powerful `bulk_domain_validator.rs` module designed for extreme concurrency via `tokio::sync::Semaphore`. 

This bulk engine can ingest thousands of domains, automatically skip false positives (using hardcoded `SKIP_PATTERNS` like Azure CDNs and `INTERNAL_PATTERNS` like RFC1918 IPs), and rapidly output valid/invalid states along with HTTP and DNS health.

## Missing Engine Controls & Visualization
1. **Bulk Upload Interface**: The UI needs a drag-and-drop or textbox input to accept hundreds of domains at once.
2. **Concurrency Slider**: The user should be able to adjust the `max_concurrency` integer passed to the `validate_domains_bulk` function based on their network bandwidth.
3. **Live Stats Dashboard**: The UI needs to render the `ValidationStats` object in real-time, showing `domains_per_sec`, `success_rate`, and counts for skipped/DNS-failed/HTTP-failed.
4. **Skip Pattern Overrides**: Allow users to see and optionally disable the hardcoded skip patterns (e.g., if they *want* to scan `.cdn.cloudflare.net` targets).

## Implementation Request
1. Build a new "Bulk Operations" module in the Svelte frontend.
2. Implement a streaming table that updates as `ValidationResult` objects are yielded by the concurrent workers.
3. Add a CSV export function for the validated output.
